use std::collections::BTreeMap;
use crate::common::with_io_output;
use star_setup::{
  config::Config,
  profile::{insert_profile, list_profiles, Profile},
};

#[test]
fn test_list_profiles_empty() {
  let ((), out) = with_io_output(|io| {
    let config = Config::new();
    list_profiles(&config, io);
  });
  assert!(out.contains("No profiles configured"));
}

#[test]
fn test_list_profiles_with_entries() {
  let ((), out) = with_io_output(|io| {
    let mut config = Config::new();
    insert_profile(
      &mut config,
      "myprofile",
      Profile {
        test_repos: BTreeMap::new(),
        deps: BTreeMap::from([("default".to_string(), vec!["user/repo1".to_string()])]),
      },
    );
    list_profiles(&config, io);
  });
  assert!(out.contains("myprofile"));
  assert!(out.contains("user/repo1"));
}
