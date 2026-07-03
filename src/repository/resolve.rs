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
