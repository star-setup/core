use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Profile {
  pub test_repos: BTreeMap<String, String>,
  pub deps: BTreeMap<String, Vec<String>>,
}

impl Profile {
  /// # Errors
  /// Returns an error if neither a test repo nor any dependency is given.
  pub fn from_args(
    test_repos: BTreeMap<String, String>,
    deps: BTreeMap<String, Vec<String>>,
  ) -> Result<Self, String> {
    if test_repos.is_empty() && deps.is_empty() {
      return Err("profile requires --test-repo or at least one dependency repo".to_string());
    }
    Ok(Self { test_repos, deps })
  }
}
