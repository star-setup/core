use crate::repository::repo_dir_name;
use std::{collections::HashSet, iter::once};

/// Builds the full ordered list of repositories, deduplicating by directory name.
#[must_use]
pub fn build_repo_list(test_repo: &str, deps: &[String]) -> Vec<String> {
  let mut seen = HashSet::new();
  once(test_repo.to_string())
    .chain(deps.iter().cloned())
    .filter(|r| seen.insert(repo_dir_name(r)))
    .collect()
}
