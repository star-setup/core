use crate::{ctx::RunCtx, utils::dry_run_or_do, workspace::Workspace};
use std::fs;

impl Workspace {
  /// Removes the build directory.
  /// # Errors
  /// Returns an error if the build directory cannot be removed.
  pub fn clean(&self, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
    writeln!(ctx.io.output, "Cleaning workspace").ok();

    if self.root.join("package.json").exists() {
      let node_modules = self.root.join("node_modules");
      if !node_modules.exists() {
        writeln!(
          ctx.io.output,
          "  node_modules does not exist: {}",
          node_modules.display()
        )
        .ok();
        return Ok(());
      }
      dry_run_or_do(
        "remove directory",
        "Removing",
        &node_modules,
        ctx,
        "Clean",
        || {
          fs::remove_dir_all(&node_modules)
            .map_err(|e| format!("Failed to remove node_modules: {e}"))
        },
      )?;
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
      ctx,
      "Clean",
      || {
        fs::remove_dir_all(&self.build_path)
          .map_err(|e| format!("Failed to remove build directory: {e}"))
      },
    )?;

    Ok(())
  }
}
