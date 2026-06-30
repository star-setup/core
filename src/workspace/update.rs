use crate::{ctx::RunCtx, repository::pull_repository, workspace::Workspace};

impl Workspace {
  /// Pulls latest changes for all repositories.
  /// # Errors
  /// Returns an error if any `git pull` command fails.
  pub fn update(&self, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
    writeln!(
      ctx.io.output,
      "Updating {} repositories\n",
      self.repo_dirs.len()
    )
    .ok();

    let mut errors: Vec<String> = Vec::new();

    for repo_dir in &self.repo_dirs {
      let name = repo_dir
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();

      if ctx.flags.dry_run {
        writeln!(ctx.io.output, "Would update {name}").ok();
        continue;
      }

      writeln!(ctx.io.output, "  Updating {name}").ok();
      if let Err(e) = pull_repository(repo_dir, ctx) {
        writeln!(ctx.io.output, "  Failed to update {name}: {e}").ok();
        errors.push(format!("{name}: {e}"));
      }
    }

    if errors.is_empty() {
      writeln!(ctx.io.output, "\nDone").ok();
      Ok(())
    } else {
      Err(format!(
        "{} repository(s) failed to update:\n{}",
        errors.len(),
        errors.join("\n")
      ))
    }
  }
}
