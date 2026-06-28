//! Repository functions including cloning and URL resolution.

use crate::ctx::RunCtx;
use std::path::Path;

/// Converts a repository path or URL to a local directory name (`owner-repo`).
#[must_use]
pub fn repo_dir_name(path: &str) -> String {
  let clean = path.trim_end_matches('/').trim_end_matches(".git");
  let mut parts = clean.rsplit('/');
  let repo = parts.next().unwrap_or(clean);
  match parts.next() {
    Some(owner) => {
      let owner = owner.rsplit_once(':').map_or(owner, |(_, o)| o);
      format!("{owner}-{repo}")
    }
    None => clean.to_string(),
  }
}

/// Converts repository input to a full GitHub URL.
/// Accepts either 'username/repo' shorthand or a full URL.
#[must_use]
pub fn resolve_repo_url(repo_input: &str, use_ssh: bool) -> String {
  if repo_input.starts_with("http") || repo_input.starts_with("git@") {
    return repo_input.to_string();
  }
  let clean = repo_input.trim_end_matches('/').trim_end_matches(".git");
  if use_ssh {
    format!("git@github.com:{clean}.git")
  } else {
    format!("https://github.com/{clean}.git")
  }
}

/// Clones a single repository into the target directory.
/// Skips if the repository already exists.
/// # Errors
/// Returns an error if the git clone command fails
pub fn clone_repository(
  repo_path: &str,
  target_dir: &Path,
  use_ssh: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let repo_name = repo_dir_name(repo_path);
  let repo_dir = target_dir.join(&repo_name);

  if repo_dir.exists() {
    writeln!(ctx.io.output, "\n  {repo_name} already exists").ok();
    return Ok(());
  }

  writeln!(ctx.io.output, "\n  Cloning {repo_name}").ok();
  let repo_url = resolve_repo_url(repo_path, use_ssh);

  ctx
    .runner
    .run(
      &["git", "clone", &repo_url, &repo_name],
      Some(target_dir),
      &mut ctx.io,
    )
    .map_err(|e| format!("Failed to clone {repo_path}: {e}"))
}
