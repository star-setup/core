use crate::{ctx::RunCtx, utils::dry_run_or_do, workspace::Workspace};
use std::fs::remove_dir_all;

impl Workspace {
  /// Removes the build directory.
  /// # Errors
  /// Returns an error if the build directory cannot be removed.
  pub fn clean(&self, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
    writeln!(ctx.io.output, "Cleaning workspace").ok();

    if self.root.join("package.json").exists() {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Clean has no effect for npm workspaces").ok();
      }
      return Ok(());
    }

    if !self.build_path.exists() {
      writeln!(
        ctx.io.output,
        "  Build directory does not exist: {}",
        self.build_path.display()
      )
      .ok();
      return Ok(());
    }

    dry_run_or_do(
      "remove directory",
      "Removing",
      &self.build_path,
      &mut ctx.io,
      ctx.flags,
      "Clean",
      || {
        remove_dir_all(&self.build_path)
          .map_err(|e| format!("Failed to remove build directory: {e}"))
      },
    )?;

    Ok(())
  }
}
