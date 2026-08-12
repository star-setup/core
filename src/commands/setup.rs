use crate::{ctx::RunCtx, resolve::ResolvedArgs, utils::dry_run_or_do};
use std::{
  fs::{create_dir_all, remove_dir_all},
  path::Path,
};

/// Prepares the build directory, optionally cleaning it first.
/// # Errors
/// Returns an error if the build directory cannot be created or removed.
pub fn prepare_build_dir(
  build_path: &Path,
  clean: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  if clean {
    writeln!(ctx.io.output, "Cleaning build directory").ok();
    dry_run_or_do(
      "remove directory",
      "Removing",
      build_path,
      &mut ctx.io,
      ctx.flags,
      "Clean",
      || {
        if build_path.exists() {
          remove_dir_all(build_path).map_err(|e| e.to_string())
        } else {
          Ok(())
        }
      },
    )?;
    if ctx.flags.verbose {
      writeln!(ctx.io.output, "  Finished cleaning\n").ok();
    }
  }

  writeln!(ctx.io.output, "Creating build directory").ok();
  dry_run_or_do(
    "create directory",
    "Creating",
    build_path,
    &mut ctx.io,
    ctx.flags,
    "Create build directory",
    || create_dir_all(build_path).map_err(|e| e.to_string()),
  )?;

  if ctx.flags.verbose {
    writeln!(ctx.io.output, "  Finished creating\n").ok();
  }
  Ok(())
}

/// Extracts and sanitizes the repository input from args.
/// # Errors
/// Returns an error if no repository is specified.
pub fn extract_repo_input(args: &ResolvedArgs) -> Result<&str, String> {
  args
    .mono
    .test_repos
    .first()
    .map(|r| r.trim_end_matches('/'))
    .ok_or_else(|| "No repository specified".to_string())
}
