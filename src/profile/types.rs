use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Profile {
  pub test_repo: Option<String>,
  pub deps: Vec<String>,
}
