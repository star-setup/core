use crate::cli::ResolvedArgs;
use crate::commands::build::cmake_build;
use crate::commands::mono::config::create_mono_repo_cmakelists;
use crate::commands::mono::resolve::{resolve_repos_for_mono, resolve_test_repo};
use crate::config::SetupConfig;
use crate::repository::{clone_repository, repo_dir_name};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Clones and configures a mono-repo ecosystem from a profile or explicit repository list.
/// # Errors
/// Returns an error if no repository is specified, directory creation fails, or any git or `CMake` command fails.
pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &SetupConfig,
  output: &mut impl Write,
) -> Result<(), String> {
  let repo_input = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_input = repo_input.trim_end_matches('/');

  let test_repo = resolve_test_repo(repo_input)?;
  let test_repo_name = repo_dir_name(&test_repo);

  let mut repos: Vec<String> = resolve_repos_for_mono(args, config, &test_repo, output)?;

  if !repos
    .iter()
    .any(|r| repo_dir_name(r) == repo_dir_name(&test_repo))
  {
    repos.push(test_repo.clone());
  }

  writeln!(output, "Total repositories: {}\n", repos.len()).ok();

  let mono_repo_path = PathBuf::from(&args.mono.mono_dir);
  writeln!(output, "Creating directory: {}\n", mono_repo_path.display()).ok();
  fs::create_dir_all(&mono_repo_path).map_err(|e| e.to_string())?;

  writeln!(output, "Cloning repositories").ok();
  for repo in &repos {
    clone_repository(
      repo,
      &mono_repo_path,
      args.connection.ssh,
      args.connection.verbose,
      output,
    )?;
  }
  writeln!(
    output,
    "\n  Finished cloning ({} repositories)\n",
    repos.len()
  )
  .ok();

  writeln!(output, "Creating mono-repo configuration").ok();
  create_mono_repo_cmakelists(&mono_repo_path, &test_repo_name, &repos, output)?;

  writeln!(output, "Creating build directory\n").ok();
  let build_path = mono_repo_path.join(&args.build.build_dir);

  if args.build.clean && build_path.exists() {
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  writeln!(
    output,
    "Configuring with CMake in {}\n",
    build_path.display()
  )
  .ok();
  cmake_build(args, build_path.as_path(), true, output)?;

  writeln!(output, "Setup complete").ok();
  writeln!(
    output,
    "Repositories in: {}",
    dunce::canonicalize(&mono_repo_path)
      .unwrap_or(mono_repo_path)
      .display()
  )
  .ok();
  writeln!(
    output,
    "Build output in: {}",
    dunce::canonicalize(&build_path)
      .unwrap_or(build_path)
      .display()
  )
  .ok();
  Ok(())
}
