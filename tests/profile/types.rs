use serde_json::from_str;
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

#[test]
fn test_profile_deserializes_with_missing_fields() {
  let profile: Profile = from_str(r#"{"deps": {"a": ["user/lib"]}}"#).unwrap();
  assert!(profile.test_repos.is_empty());
  assert_eq!(profile.deps["a"], vec!["user/lib"]);
}
