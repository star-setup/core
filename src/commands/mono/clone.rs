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
  crate::time!(ctx.flags.timing, ctx.io.output, "Clone", {
    for repo in repos {
      writeln!(
        ctx.io.output,
        "  Cloning {}",
        crate::repository::repo_dir_name(repo)
      )
      .ok();
      clone_repository(repo, repos_path, ssh, true, false, ctx)?;
    }
    writeln!(
      ctx.io.output,
      "  Finished cloning ({} repositories)",
      repos.len()
    )
    .ok();
    Ok::<(), String>(())
  })?;

  writeln!(ctx.io.output).ok();
  Ok(())
}
