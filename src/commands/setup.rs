use crate::{
  cli::{BuildSystem, ResolvedArgs},
  commands::build_project,
  ctx::RunCtx,
  utils::dry_run_or_do,
};
use std::{fs, path::Path};

/// Prepares the build directory, optionally cleaning it first.
/// # Errors
/// Returns an error if the build directory cannot be created or removed.
pub fn prepare_build_dir(
  build_path: &std::path::Path,
  clean: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  if clean {
    writeln!(ctx.io.output, "Cleaning build directory").ok();
    dry_run_or_do(
      "remove directory",
      "Removing",
      build_path,
      ctx,
      "Clean",
      || {
        if build_path.exists() {
          fs::remove_dir_all(build_path).map_err(|e| e.to_string())
        } else {
          Ok(())
        }
      },
    )?;
    writeln!(ctx.io.output, "  Finished cleaning\n").ok();
  }

  writeln!(ctx.io.output, "Creating build directory").ok();
  dry_run_or_do(
    "create directory",
    "Creating",
    build_path,
    ctx,
    "Create build directory",
    || fs::create_dir_all(build_path).map_err(|e| e.to_string()),
  )?;

  writeln!(ctx.io.output, "  Finished creating\n").ok();
  Ok(())
}

/// Detects the build system and runs configuration and optional build.
/// # Errors
/// Returns an error if detection or build fails.
pub fn configure_and_build(
  args: &ResolvedArgs,
  project_path: &Path,
  build_path: &Path,
  build_system: BuildSystem,
  is_mono: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  writeln!(ctx.io.output, "Configuring project").ok();
  build_project(args, build_path, project_path, build_system, is_mono, ctx)
}

/// Extracts and sanitizes the repository input from args.
/// # Errors
/// Returns an error if no repository is specified.
pub fn extract_repo_input(args: &ResolvedArgs) -> Result<&str, String> {
  args
    .repo
    .as_deref()
    .map(|r| r.trim_end_matches('/'))
    .ok_or_else(|| "No repository specified".to_string())
}
