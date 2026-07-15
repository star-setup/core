use star_setup::profile::Profile;
use std::collections::HashMap;

#[test]
fn test_profile_from_args_errors_when_empty() {
  assert!(Profile::from_args(HashMap::new(), HashMap::new()).is_err());
}

#[test]
fn test_profile_from_args_test_repo_only() {
  let test_repos = HashMap::from([("default".to_string(), "user/app".to_string())]);
  assert!(Profile::from_args(test_repos, HashMap::new()).is_ok());
}
