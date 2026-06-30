use crate::{ctx::RunCtx, workspace::Workspace};

impl Workspace {
  /// Shows the status of all repositories.
  /// # Errors
  /// Returns an error if any git command fails.
  pub fn status(&self, fetch: bool, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
    writeln!(ctx.io.output, "Workspace status:\n").ok();

    for repo_dir in &self.repo_dirs {
      let name = repo_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

      if ctx.flags.dry_run {
        writeln!(ctx.io.output, "Would show status for {name}").ok();
        continue;
      }

      if fetch {
        ctx
          .runner
          .run(&["git", "fetch"], Some(repo_dir), &ctx.flags, ctx.io.output)?;
      }

      let branch = ctx
        .runner
        .run_capture(
          &["git", "rev-parse", "--abbrev-ref", "HEAD"],
          Some(repo_dir),
        )
        .unwrap_or_else(|_| "(unknown)".to_string());

      let dirty = !ctx
        .runner
        .run_capture(&["git", "status", "--porcelain"], Some(repo_dir))?
        .is_empty();

      let status_str = if dirty { "dirty" } else { "clean" };

      let ahead_behind = if fetch {
        let ahead = ctx
          .runner
          .run_capture(
            &["git", "rev-list", "--count", "@{u}..HEAD"],
            Some(repo_dir),
          )
          .unwrap_or_else(|_| "?".to_string());
        let behind = ctx
          .runner
          .run_capture(
            &["git", "rev-list", "--count", "HEAD..@{u}"],
            Some(repo_dir),
          )
          .unwrap_or_else(|_| "?".to_string());
        format!("  ↑{ahead} ↓{behind}")
      } else {
        String::new()
      };

      writeln!(
        ctx.io.output,
        "  {name:<20} {branch:<12} {status_str}{ahead_behind}"
      )
      .ok();
    }

    Ok(())
  }
}
