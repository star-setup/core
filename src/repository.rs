//! Repository functions including cloning and URL resolution.

use std::path::Path;
use crate::utils::run_command;

/// Converts repository input to a full GitHub URL.
/// Accepts either 'username/repo' shorthand or a full URL.
pub fn resolve_repo_url(repo_input: &str, use_ssh: bool) -> String {
  if repo_input.starts_with("http") || repo_input.starts_with("git@") {
    return repo_input.to_string();
  }
  if use_ssh { format!("git@github.com:{}.git",     repo_input) }
  else       { format!("https://github.com/{}.git", repo_input) }
}

/// Clones a single repository into the target directory.
/// Skips if the repository already exists.
pub fn clone_repository(
  repo_path: &str,
  target_dir: &Path,
  use_ssh: bool, verbose: bool
) -> Result<(), String> {
  let repo_name = repo_path.split('/').last().unwrap_or(repo_path).trim_end_matches(".git");
  let repo_dir  = target_dir.join(repo_name);

  if repo_dir.exists() {
    println!("\n  {} already exists", repo_name);
    return Ok(());
  }

  println!("\n  Cloning {}", repo_name);
  let repo_url   = resolve_repo_url(repo_path, use_ssh);
  let target_str = target_dir.to_str().ok_or("Invalid target directory path")?;

  run_command(&["git", "clone", &repo_url], Some(target_str), verbose)
    .map_err(|e| format!("Failed to clone {}: {}", repo_path, e))
}
