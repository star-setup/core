use crate::{
  cli::{
    build::{detect_mono_build_system, BuildSystem},
    ResolvedArgs,
  },
  commands::{
    build::build_project,
    mono::{
      config::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild},
      resolve::{resolve_repos_for_mono, resolve_test_repo},
      wraps::hoist_wraps,
    },
  },
  config::types::SetupConfig,
  repository::{clone_repository, repo_dir_name},
};
use std::{
  fs,
  io::{BufRead, Write},
  path::PathBuf,
};

fn clone_mono_repos(
  repos: &[String],
  repos_path: &std::path::Path,
  ssh: bool,
  verbose: bool,
  timing: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  writeln!(output, "Cloning repositories").ok();
  let t = std::time::Instant::now();
  for repo in repos {
    clone_repository(repo, repos_path, ssh, verbose, output)?;
  }
  writeln!(
    output,
    "\n  Finished cloning ({} repositories)\n",
    repos.len()
  )
  .ok();
  if timing {
    writeln!(output, "  [timing] Clone: {:.2?}", t.elapsed()).ok();
  }
  Ok(())
}

fn generate_mono_config(
  build_system: &BuildSystem,
  mono_repo_path: &std::path::Path,
  repos_path: &std::path::Path,
  repo_dirs: &[PathBuf],
  repos: &[String],
  output: &mut impl Write,
) -> Result<Option<std::collections::HashMap<String, String>>, String> {
  writeln!(output, "Creating mono-repo configuration").ok();
  match build_system {
    BuildSystem::Cmake => {
      create_mono_repo_cmakelists(mono_repo_path, repos, output)?;
      Ok(None)
    }
    BuildSystem::Meson => {
      let map = hoist_wraps(repos_path, repo_dirs, output)?;
      let subproject_names: Vec<String> = repos
        .iter()
        .map(|r| {
          let dir = repo_dir_name(r);
          map
            .iter()
            .find(|(_, v)| *v == &dir)
            .map(|(k, _)| k.clone())
            .unwrap_or(dir)
        })
        .collect();
      create_mono_repo_mesonbuild(mono_repo_path, &subproject_names, output)?;
      Ok(Some(map))
    }
  }
}

#[must_use]
pub fn build_repo_list(test_repo: &str, deps: &[String]) -> Vec<String> {
  let mut seen = std::collections::HashSet::new();
  std::iter::once(test_repo.to_string())
    .chain(deps.iter().cloned())
    .filter(|r| seen.insert(repo_dir_name(r)))
    .collect()
}

fn prepare_build_dir(
  build_path: &std::path::Path,
  clean: bool,
  timing: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  if clean && build_path.exists() {
    let t = std::time::Instant::now();
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(build_path).map_err(|e| e.to_string())?;
    if timing {
      writeln!(output, "  [timing] Clean: {:.2?}", t.elapsed()).ok();
    }
  }

  let t = std::time::Instant::now();
  writeln!(output, "Creating build directory\n").ok();
  fs::create_dir_all(build_path).map_err(|e| e.to_string())?;
  if timing {
    writeln!(
      output,
      "  [timing] Create build directory: {:.2?}",
      t.elapsed()
    )
    .ok();
  }
  Ok(())
}

/// Clones and configures a mono-repo ecosystem from a profile or explicit repository list.
/// # Errors
/// Returns an error if no repository is specified, directory creation fails, or any build system command fails.
pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &SetupConfig,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  let timing = args.diagnostic.timing;
  let total = std::time::Instant::now();

  let repo_input = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_input = repo_input.trim_end_matches('/');

  let test_repo = resolve_test_repo(repo_input)?;
  let deps = resolve_repos_for_mono(args, config, &test_repo, output)?;
  let repos = build_repo_list(&test_repo, &deps);
  writeln!(output, "Total repositories: {}\n", repos.len()).ok();

  let t = std::time::Instant::now();
  let mono_repo_path = PathBuf::from(&args.mono.mono_dir);
  writeln!(output, "Creating directory: {}\n", mono_repo_path.display()).ok();
  fs::create_dir_all(&mono_repo_path).map_err(|e| e.to_string())?;
  if timing {
    writeln!(output, "  [timing] Create directory: {:.2?}", t.elapsed()).ok();
  }

  let repos_path = mono_repo_path.join("repos");
  fs::create_dir_all(&repos_path).map_err(|e| e.to_string())?;

  clone_mono_repos(
    &repos,
    &repos_path,
    args.connection.ssh,
    args.connection.verbose,
    timing,
    output,
  )?;

  let repo_dirs: Vec<PathBuf> = repos
    .iter()
    .map(|r| repos_path.join(repo_dir_name(r)))
    .collect();

  let build_system = detect_mono_build_system(&repo_dirs, input, output, timing)?;

  let canonical_map = generate_mono_config(
    &build_system,
    &mono_repo_path,
    &repos_path,
    &repo_dirs,
    &repos,
    output,
  )?;

  let build_path = mono_repo_path.join(&args.build.build_dir);
  prepare_build_dir(build_path.as_path(), args.build.clean, timing, output)?;

  writeln!(output, "Configuring project in {}\n", build_path.display()).ok();
  build_project(
    args,
    build_path.as_path(),
    &mono_repo_path,
    true,
    input,
    output,
    timing,
  )?;

  writeln!(output, "Setup complete").ok();
  writeln!(
    output,
    "Repositories in: {}",
    dunce::canonicalize(&mono_repo_path)
      .unwrap_or(mono_repo_path)
      .display()
  )
  .ok();
  if let Some(map) = canonical_map {
    let test_repo_name = repo_dir_name(&test_repo);
    if let Some((canonical, _)) = map.iter().find(|(_, v)| *v == &test_repo_name) {
      let exe_name = if cfg!(windows) {
        format!("{canonical}.exe")
      } else {
        canonical.clone()
      };
      let exe_path = build_path
        .join("repos")
        .join(&test_repo_name)
        .join(&exe_name);
      writeln!(
        output,
        "Executable: {}",
        dunce::canonicalize(&exe_path).unwrap_or(exe_path).display()
      )
      .ok();
    }
  } else {
    writeln!(
      output,
      "Build output in: {}",
      dunce::canonicalize(&build_path)
        .unwrap_or(build_path)
        .display()
    )
    .ok();
  }
  if timing {
    writeln!(output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
  Ok(())
}
