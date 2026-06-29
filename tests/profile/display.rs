use crate::common::with_io_output;
use star_setup::{
  config::SetupConfig,
  profile::{insert_profile, list_profiles},
};

#[test]
fn test_list_profiles_empty() {
  let ((), out) = with_io_output(|io| {
    let config = SetupConfig::new();
    list_profiles(&config, io);
  });
  assert!(out.contains("No profiles configured"));
}

#[test]
fn test_list_profiles_with_entries() {
  let ((), out) = with_io_output(|io| {
    let mut config = SetupConfig::new();
    insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
    list_profiles(&config, io);
  });
  assert!(out.contains("myprofile"));
  assert!(out.contains("user/repo1"));
}
