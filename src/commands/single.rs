use crate::{
  cli::ResolvedArgs,
  commands::{
    build::build_project,
    header::{print_mode_header, ModeHeader},
  },
  prompts::confirm,
  repository::{repo_dir_name, resolve_repo_url},
  utils::process::run_command,
};
use std::{
  fs,
  io::{BufRead, Write},
  path::{Path, PathBuf},
};

/// Clones and configures a single repository.
/// # Errors
/// Returns an error if no repository is specified, or if any git or build system fails.
pub fn single_repo_mode(
  args: &ResolvedArgs,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  let repo = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_url = resolve_repo_url(repo, args.connection.ssh);
  let dir_name = repo_dir_name(repo);

  print_mode_header(
    &ModeHeader {
      mode: "Single Repository Mode",
      test_repo: None,
      repo_name: Some(&dir_name),
      use_ssh: args.connection.ssh,
      mono_dir: None,
      profile: None,
      lib_count: None,
    },
    output,
  );

  let repo_path = Path::new(&dir_name);
  if repo_path.exists() {
    writeln!(output, "Repository {dir_name} already exists").ok();
    if confirm("Update existing repository?", args.yes, input, output)? {
      writeln!(output, "Updating {dir_name}\n").ok();
      run_command(
        &["git", "pull"],
        Some(Path::new(&dir_name)),
        args.connection.verbose,
        output,
      )?;
    }
  } else {
    writeln!(output, "Cloning {dir_name}\n").ok();
    run_command(
      &["git", "clone", &repo_url, &dir_name],
      None,
      args.connection.verbose,
      output,
    )?;
  }

  let build_path = PathBuf::from(&dir_name).join(&args.build.build_dir);
  if args.build.clean && build_path.exists() {
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  writeln!(
    output,
    "Creating build directory: {}\n",
    args.build.build_dir
  )
  .ok();
  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  writeln!(output, "Configuring project\n").ok();
  build_project(
    args,
    build_path.as_path(),
    Path::new(&dir_name),
    false,
    input,
    output,
  )?;

  writeln!(
    output,
    "Project finished in {dir_name}/{}",
    args.build.build_dir
  )
  .ok();
  Ok(())
}
