use star_setup::config::{save_config, SetupConfig};
use star_setup::profiles::{
  has_profile, insert_profile, remove_profile_entry,
};

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
  let tmp = std::env::temp_dir().join("star_setup_test_add_profile");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());

  let args = vec!["myprofile".to_string(), "user/repo1".to_string()];
  star_setup::profiles::add_profile(&mut config, &args, true).unwrap();
  assert!(has_profile(&config, "myprofile"));
  assert!(path.exists());

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_remove_profile_removes_and_saves() {
  let tmp = std::env::temp_dir().join("star_setup_test_remove_profile");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
  save_config(&mut config).unwrap();

  star_setup::profiles::remove_profile(&mut config, "myprofile", true).unwrap();
  assert!(!has_profile(&config, "myprofile"));

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_remove_profile_not_found() {
  let mut config = SetupConfig::new();
  star_setup::profiles::remove_profile(&mut config, "nonexistent", true).unwrap();
}

#[test]
fn test_add_profile_errors_on_insufficient_args() {
  let mut config = SetupConfig::new();
  let args = vec!["myprofile".to_string()];
  let result = star_setup::profiles::add_profile(&mut config, &args, true);
  assert!(result.is_err());
}

#[test]
fn test_add_profile_errors_on_empty_args() {
  let mut config = SetupConfig::new();
  let result = star_setup::profiles::add_profile(&mut config, &[], true);
  assert!(result.is_err());
}

#[test]
fn test_add_profile_overwrites_existing() {
  let tmp = std::env::temp_dir().join("star_setup_test_profile_overwrite");
  std::fs::create_dir_all(&tmp).ok();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.join(".star-setup.json"));
  insert_profile(&mut config, "myprofile", vec!["old/repo".to_string()]);

  let args = vec!["myprofile".to_string(), "new/repo".to_string()];
  star_setup::profiles::add_profile(&mut config, &args, true).unwrap();
  assert_eq!(config.profiles["myprofile"], vec!["new/repo"]);

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_add_profile_multiple_repos() {
  let tmp = std::env::temp_dir().join("star_setup_test_profile_multi");
  std::fs::create_dir_all(&tmp).ok();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.join(".star-setup.json"));

  let args = vec![
    "myprofile".to_string(),
    "user/repo1".to_string(),
    "user/repo2".to_string(),
    "user/repo3".to_string(),
  ];
  star_setup::profiles::add_profile(&mut config, &args, true).unwrap();
  assert_eq!(config.profiles["myprofile"].len(), 3);

  std::fs::remove_dir_all(&tmp).ok();
}
