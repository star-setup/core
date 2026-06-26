use super::common::{empty_input, make_io, sink};
use star_setup::{
  config::SetupConfig,
  profile::{insert_profile, list_profiles},
};

#[test]
fn test_list_profiles_empty() {
  let config = SetupConfig::new();

  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  list_profiles(&config, &mut io);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("No profiles configured"));
}

#[test]
fn test_list_profiles_with_entries() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);

  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  list_profiles(&config, &mut io);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("myprofile"));
  assert!(out.contains("user/repo1"));
}
