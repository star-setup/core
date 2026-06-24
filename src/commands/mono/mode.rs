use crate::cli::{detect_mono_build_system, BuildSystem, ResolvedArgs};
use crate::commands::build::{cmake_build, meson_build};
use crate::commands::mono::config::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild};
use crate::commands::mono::resolve::{resolve_repos_for_mono, resolve_test_repo};
use crate::commands::mono::wraps::hoist_wraps;
use crate::config::types::SetupConfig;
use crate::repository::{clone_repository, repo_dir_name};
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

fn clone_mono_repos(
  repos: &[String],
  repos_path: &std::path::Path,
  ssh: bool,
  verbose: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  for repo in repos {
    clone_repository(repo, repos_path, ssh, verbose, output)?;
  }
  writeln!(
    output,
    "\n  Finished cloning ({} repositories)\n",
    repos.len()
  )
  .ok();
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

/// Clones and configures a mono-repo ecosystem from a profile or explicit repository list.
/// # Errors
/// Returns an error if no repository is specified, directory creation fails, or any build system command fails.
pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &SetupConfig,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  let repo_input = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_input = repo_input.trim_end_matches('/');

  let test_repo = resolve_test_repo(repo_input)?;
  let test_repo_name = repo_dir_name(&test_repo);

  let mut repos: Vec<String> = resolve_repos_for_mono(args, config, &test_repo, output)?
    .iter()
    .filter(|r| repo_dir_name(r) != test_repo_name)
    .cloned()
    .collect();
  repos.dedup_by(|a, b| repo_dir_name(a) == repo_dir_name(b));
  repos.insert(0, test_repo.clone());
  writeln!(output, "Total repositories: {}\n", repos.len()).ok();

  let mono_repo_path = PathBuf::from(&args.mono.mono_dir);
  writeln!(output, "Creating directory: {}\n", mono_repo_path.display()).ok();
  fs::create_dir_all(&mono_repo_path).map_err(|e| e.to_string())?;

  let repos_path = mono_repo_path.join("repos");
  fs::create_dir_all(&repos_path).map_err(|e| e.to_string())?;

  writeln!(output, "Cloning repositories").ok();
  clone_mono_repos(
    &repos,
    &repos_path,
    args.connection.ssh,
    args.connection.verbose,
    output,
  )?;

  let repo_dirs: Vec<PathBuf> = repos
    .iter()
    .map(|r| repos_path.join(repo_dir_name(r)))
    .collect();

  writeln!(output, "Detecting build system\n").ok();
  let build_system = detect_mono_build_system(&repo_dirs, input, output)?;

  writeln!(output, "Creating mono-repo configuration").ok();
  let canonical_map = generate_mono_config(
    &build_system,
    &mono_repo_path,
    &repos_path,
    &repo_dirs,
    &repos,
    output,
  )?;

  writeln!(output, "Creating build directory\n").ok();
  let build_path = mono_repo_path.join(&args.build.build_dir);

  if args.build.clean && build_path.exists() {
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  writeln!(output, "Configuring project in {}\n", build_path.display()).ok();
  match &build_system {
    BuildSystem::Cmake => cmake_build(args, build_path.as_path(), true, output)?,
    BuildSystem::Meson => meson_build(args, build_path.as_path(), &mono_repo_path, output)?,
  }

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
  Ok(())
}
