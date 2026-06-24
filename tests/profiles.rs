use star_setup::config::io::{load_config, save_config};
use star_setup::config::types::SetupConfig;
use star_setup::profiles::{has_profile, insert_profile, remove_profile_entry};
mod common;
use common::{empty_input, sink};

#[test]
fn test_insert_profile() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
  assert!(config.profiles.contains_key("myprofile"));
}

#[test]
fn test_has_profile_true() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec![]);
  assert!(has_profile(&config, "myprofile"));
}

#[test]
fn test_has_profile_false() {
  let config = SetupConfig::new();
  assert!(!has_profile(&config, "nonexistent"));
}

#[test]
fn test_remove_profile_entry_exists() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec![]);
  assert!(remove_profile_entry(&mut config, "myprofile"));
  assert!(!config.profiles.contains_key("myprofile"));
}

#[test]
fn test_remove_profile_entry_missing() {
  let mut config = SetupConfig::new();
  assert!(!remove_profile_entry(&mut config, "nonexistent"));
}

#[test]
fn test_add_profile_inserts_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());

  let args = vec!["myprofile".to_string(), "user/repo1".to_string()];
  star_setup::profiles::add_profile(&mut config, &args, true, &mut empty_input(), &mut sink())
    .unwrap();
  assert!(has_profile(&config, "myprofile"));
  assert!(path.exists());
}

#[test]
fn test_remove_profile_removes_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
  save_config(&mut config).unwrap();

  star_setup::profiles::remove_profile(
    &mut config,
    "myprofile",
    true,
    &mut empty_input(),
    &mut sink(),
  )
  .unwrap();
  assert!(!has_profile(&config, "myprofile"));
}

#[test]
fn test_remove_profile_not_found() {
  let mut config = SetupConfig::new();
  star_setup::profiles::remove_profile(
    &mut config,
    "nonexistent",
    true,
    &mut empty_input(),
    &mut sink(),
  )
  .unwrap();
}

#[test]
fn test_add_profile_errors_on_insufficient_args() {
  let mut config = SetupConfig::new();
  let args = vec!["myprofile".to_string()];
  let result =
    star_setup::profiles::add_profile(&mut config, &args, true, &mut empty_input(), &mut sink());
  assert!(result.is_err());
}

#[test]
fn test_add_profile_errors_on_empty_args() {
  let mut config = SetupConfig::new();
  let result =
    star_setup::profiles::add_profile(&mut config, &[], true, &mut empty_input(), &mut sink());
  assert!(result.is_err());
}

#[test]
fn test_add_profile_overwrites_existing() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_profile(&mut config, "myprofile", vec!["old/repo".to_string()]);

  let args = vec!["myprofile".to_string(), "new/repo".to_string()];
  star_setup::profiles::add_profile(&mut config, &args, true, &mut empty_input(), &mut sink())
    .unwrap();
  assert_eq!(config.profiles["myprofile"], vec!["new/repo"]);
}

#[test]
fn test_add_profile_multiple_repos() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));

  let args = vec![
    "myprofile".to_string(),
    "user/repo1".to_string(),
    "user/repo2".to_string(),
    "user/repo3".to_string(),
  ];
  star_setup::profiles::add_profile(&mut config, &args, true, &mut empty_input(), &mut sink())
    .unwrap();
  assert_eq!(config.profiles["myprofile"].len(), 3);
}

#[test]
fn test_add_profile_aborts_when_exists_and_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_profile(&mut config, "myprofile", vec!["old/repo".to_string()]);

  let input = b"n\n";
  let args = vec!["myprofile".to_string(), "new/repo".to_string()];
  star_setup::profiles::add_profile(&mut config, &args, false, &mut input.as_ref(), &mut sink())
    .unwrap();
  assert_eq!(config.profiles["myprofile"], vec!["old/repo"]);
}

#[test]
fn test_remove_profile_aborts_when_not_confirmed() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);

  let input = b"n\n";
  star_setup::profiles::remove_profile(
    &mut config,
    "myprofile",
    false,
    &mut input.as_ref(),
    &mut sink(),
  )
  .unwrap();
  assert!(has_profile(&config, "myprofile"));
}

#[test]
fn test_list_profiles_empty() {
  let config = SetupConfig::new();
  let mut output = sink();
  star_setup::profiles::list_profiles(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("No profiles configured"));
}

#[test]
fn test_list_profiles_with_entries() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
  let mut output = sink();
  star_setup::profiles::list_profiles(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("myprofile"));
  assert!(out.contains("user/repo1"));
}

#[test]
fn test_save_and_load_profile_roundtrip() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_profile(
    &mut config,
    "myprofile",
    vec!["user/repo1".to_string(), "user/repo2".to_string()],
  );
  save_config(&mut config).unwrap();

  let loaded = load_config(&[path], &mut sink());
  assert!(loaded.profiles.contains_key("myprofile"));
  assert_eq!(
    loaded.profiles["myprofile"],
    vec!["user/repo1", "user/repo2"]
  );
}
