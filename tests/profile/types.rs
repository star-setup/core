use star_setup::profile::Profile;
use std::collections::BTreeMap;

#[test]
fn test_profile_from_args_errors_when_empty() {
  assert!(Profile::from_args(BTreeMap::new(), BTreeMap::new()).is_err());
}

#[test]
fn test_profile_from_args_test_repo_only() {
  let test_repos = BTreeMap::from([("default".to_string(), "user/app".to_string())]);
  assert!(Profile::from_args(test_repos, BTreeMap::new()).is_ok());
}
