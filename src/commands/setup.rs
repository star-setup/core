use crate::{
  cli::{BuildSystem, ResolvedArgs},
  commands::build_project,
  ctx::RunCtx,
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
  if clean && ctx.io.dry_run {
    writeln!(ctx.io.output, "Cleaning build directory\n").ok();
    writeln!(
      ctx.io.output,
      "Would remove directory: {}",
      build_path.display()
    )
    .ok();
  } else if clean && build_path.exists() {
    writeln!(ctx.io.output, "Cleaning build directory\n").ok();
    crate::time!(ctx.io.timing, ctx.io.output, "Clean", {
      fs::remove_dir_all(build_path).map_err(|e| e.to_string())?;
    });
  }

  writeln!(ctx.io.output, "Creating build directory\n").ok();
  if ctx.io.dry_run {
    writeln!(
      ctx.io.output,
      "Would create directory: {}",
      build_path.display()
    )
    .ok();
  } else {
    crate::time!(ctx.io.timing, ctx.io.output, "Create build directory", {
      fs::create_dir_all(build_path).map_err(|e| e.to_string())?;
    });
  }
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
  writeln!(ctx.io.output, "Configuring project\n").ok();
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
