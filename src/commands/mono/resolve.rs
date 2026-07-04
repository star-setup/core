use crate::{cli::ResolvedArgs, config::SetupConfig, ctx::IoCtx, profile::list_profiles};

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

/// Resolves the list of repositories for mono-repo mode from a profile or explicit repo list.
/// # Errors
/// Returns an error if the specified profile does not exist, or has no repositories.
pub fn resolve_repos_for_mono(
  args: &ResolvedArgs,
  config: &SetupConfig,
  io: &mut IoCtx<'_>,
) -> Result<Vec<String>, String> {
  if let Some(profile_name) = &args.mono.profile {
    let profile_repos = config.profiles.get(profile_name).ok_or_else(|| {
      list_profiles(config, io);
      format!("Profile '{profile_name}' not found")
    })?;
    if profile_repos.test_repo.is_none() && profile_repos.deps.is_empty() {
      return Err(format!("Profile '{profile_name}' has no repositories"));
    }
    Ok(profile_repos.deps.clone())
  } else if let Some(r) = &args.mono.repos {
    Ok(r.clone())
  } else {
    Err("No repos or profile specified for mono-repo mode".to_string())
  }
}
