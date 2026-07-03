use crate::ctx::RunCtx;
use std::path::Path;

/// Pulls the latest changes for an existing repository.
/// # Errors
/// Returns an error if the `git pull` command fails.
pub fn pull_repo(repo_path: &Path, ctx: &mut RunCtx<'_, '_>) -> Result<(), String> {
  ctx
    .runner
    .run(&["git", "pull"], Some(repo_path), ctx.flags, ctx.io.output)
}
