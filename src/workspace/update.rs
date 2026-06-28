use crate::{ctx::RunCtx, workspace::resolve::Workspace};

/// Pulls latest changes for all repositories in the workspace.
/// # Errors
/// Returns an error if any `git pull` command fails.
pub fn update_workspace(workspace: &Workspace, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
  writeln!(
    ctx.io.output,
    "Updating {} repositories\n",
    workspace.repo_dirs.len()
  )
  .ok();

  let mut errors: Vec<String> = Vec::new();

  for repo_dir in &workspace.repo_dirs {
    let name = repo_dir
      .file_name()
      .map(|n| n.to_string_lossy())
      .unwrap_or_default();

    writeln!(ctx.io.output, "  Updating {name}").ok();
    if let Err(e) = ctx
      .runner
      .run(&["git", "pull"], Some(repo_dir), &mut ctx.io)
    {
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
