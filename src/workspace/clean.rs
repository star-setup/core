use crate::{ctx::RunCtx, workspace::resolve::Workspace};
use std::fs;

/// Removes the build directory from the workspace.
/// # Errors
/// Returns an error if the build directory cannot be removed.
pub fn clean_workspace(workspace: &Workspace, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
  if !workspace.build_path.exists() {
    writeln!(
      ctx.io.output,
      "Build directory does not exist: {}",
      workspace.build_path.display()
    )
    .ok();
    return Ok(());
  }

  writeln!(
    ctx.io.output,
    "Removing build directory: {}",
    workspace.build_path.display()
  )
  .ok();

  if ctx.io.dry_run {
    writeln!(
      ctx.io.output,
      "Would remove directory: {}",
      workspace.build_path.display()
    )
    .ok();
  } else {
    fs::remove_dir_all(&workspace.build_path)
      .map_err(|e| format!("Failed to remove build directory: {e}"))?;
    writeln!(ctx.io.output, "Done").ok();
  }

  Ok(())
}
