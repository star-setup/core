use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Profile {
  pub test_repo: Option<String>,
  pub deps: Vec<String>,
}

impl Profile {
  /// # Errors
  /// Returns an error if neither a test repo nor any dependency is given.
  pub fn from_args(test_repo: Option<String>, deps: Vec<String>) -> Result<Self, String> {
    if test_repo.is_none() && deps.is_empty() {
      return Err("profile requires --test-repo or at least one dependency repo".to_string());
    }
    Ok(Self { test_repo, deps })
  }
}
