use crate::{config::Config, profile::Profile, resolve::ResolvedArgs};

/// Normalizes a repository input to `username/repo` format.
/// # Errors
/// Returns an error if the input is not a recognizable GitHub URL or `username/repo` format.
pub fn resolve_test_repo(repo_input: &str) -> Result<String, String> {
  let repo_input = repo_input.trim_end_matches('/');
  if repo_input.starts_with("http") || repo_input.starts_with("git@") {
    if repo_input.contains("github.com/") || repo_input.contains("github.com:") {
      let parts: Vec<&str> = repo_input.split('/').collect();
      if parts.len() < 2 {
        return Err("Repository URL missing repository name".to_string());
      }
      let user = parts[parts.len() - 2].split(':').next_back().unwrap_or("");
      let repo = parts[parts.len() - 1].trim_end_matches(".git");
      Ok(format!("{user}/{repo}"))
    } else {
      Err("Could not parse repository URL".to_string())
    }
  } else if repo_input.contains('/') {
    Ok(repo_input.to_string())
  } else {
    Err("Repository must be in format 'username/repo' for mono-repo mode".to_string())
  }
}

/// Resolves the named profile for mono-repo mode, if one was requested.
#[must_use]
pub fn resolve_profile<'a>(args: &ResolvedArgs, config: &'a Config) -> Option<&'a Profile> {
  args
    .mono
    .profile
    .as_deref()
    .and_then(|name| config.profiles.get(name))
}

/// Resolves the test repo for mono-repo mode from the already-resolved repo.
/// # Errors
/// Returns an error if no repository is available.
pub fn resolve_test_repo_for_mono(args: &ResolvedArgs) -> Result<String, String> {
  match args.repo.as_deref() {
    Some(r) => resolve_test_repo(r.trim_end_matches('/')),
    None => Err("No repository specified".to_string()),
  }
}

/// Resolves the dependency repositories for mono-repo mode from a profile or explicit list.
#[must_use]
pub fn resolve_repos_for_mono(args: &ResolvedArgs, profile: Option<&Profile>) -> Vec<String> {
  profile
    .map(|p| p.deps.values().flatten().cloned().collect())
    .or_else(|| args.mono.repos.clone())
    .unwrap_or_default()
}
