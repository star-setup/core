use star_setup::profile::Profile;

#[test]
fn test_profile_from_args_errors_when_empty() {
  assert!(Profile::from_args(None, vec![]).is_err());
}

#[test]
fn test_profile_from_args_test_repo_only() {
  assert!(Profile::from_args(Some("user/app".to_string()), vec![]).is_ok());
}
