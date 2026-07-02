use crate::{ctx::RunCtx, workspace::Workspace};
use std::fs;

impl Workspace {
  /// Removes the build directory.
  /// # Errors
  /// Returns an error if the build directory cannot be removed.
  pub fn clean(&self, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
    if self.root.join("package.json").exists() {
      let node_modules = self.root.join("node_modules");
      if !node_modules.exists() {
        writeln!(
          ctx.io.output,
          "node_modules does not exist: {}",
          node_modules.display()
        )
        .ok();
        return Ok(());
      }
      if ctx.flags.dry_run {
        writeln!(
          ctx.io.output,
          "Would remove directory: {}",
          node_modules.display()
        )
        .ok();
      } else {
        writeln!(
          ctx.io.output,
          "Removing node_modules: {}",
          node_modules.display()
        )
        .ok();
        fs::remove_dir_all(&node_modules)
          .map_err(|e| format!("Failed to remove node_modules: {e}"))?;
        writeln!(ctx.io.output, "Done").ok();
      }
      return Ok(());
    }

    if !self.build_path.exists() {
      writeln!(
        ctx.io.output,
        "Build directory does not exist: {}",
        self.build_path.display()
      )
      .ok();
      return Ok(());
    }

    writeln!(
      ctx.io.output,
      "Removing build directory: {}",
      self.build_path.display()
    )
    .ok();

    if ctx.flags.dry_run {
      writeln!(
        ctx.io.output,
        "Would remove directory: {}",
        self.build_path.display()
      )
      .ok();
    } else {
      fs::remove_dir_all(&self.build_path)
        .map_err(|e| format!("Failed to remove build directory: {e}"))?;
      writeln!(ctx.io.output, "Done").ok();
    }

    Ok(())
  }
}
