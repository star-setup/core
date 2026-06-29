use crate::{ctx::RunCtx, repository::clone_repository};

/// Clones all repositories into the mono-repo directory.
/// # Errors
/// Returns an error if any repository fails to clone.
pub fn clone_mono_repos(
  repos: &[String],
  repos_path: &std::path::Path,
  ssh: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  writeln!(ctx.io.output, "Cloning repositories").ok();
  crate::time!(ctx.io.timing, ctx.io.output, "Clone", {
    for repo in repos {
      clone_repository(repo, repos_path, ssh, ctx)?;
    }
    Ok::<(), String>(())
  })?;
  writeln!(
    ctx.io.output,
    "\n  Finished cloning ({} repositories)\n",
    repos.len()
  )
  .ok();
  Ok(())
}
