use crate::repository::repo_dir_name;
use std::collections::HashSet;

/// Builds the full ordered list of repositories, deduplicating by directory name.
#[must_use]
pub fn build_repo_list(test_repos: &[String], deps: &[String]) -> Vec<String> {
  let mut seen = HashSet::new();
  test_repos
    .iter()
    .chain(deps)
    .cloned()
    .filter(|r| seen.insert(repo_dir_name(r)))
    .collect()
}
