use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Profile {
  pub test_repos: HashMap<String, String>,
  pub deps: HashMap<String, Vec<String>>,
}

impl Profile {
  /// # Errors
  /// Returns an error if neither a test repo nor any dependency is given.
  pub fn from_args(
    test_repos: HashMap<String, String>,
    deps: HashMap<String, Vec<String>>
  ) -> Result<Self, String> {
    if test_repos.is_empty() && deps.is_empty() {
      return Err("profile requires --test-repo or at least one dependency repo".to_string());
    }
    Ok(Self { test_repos, deps })
  }
}
