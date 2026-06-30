use crate::common::{make_flags, with_io_dir, with_io_input_output, with_io_output};
use star_setup::{
  config::{load_config, save_config, SetupConfig},
  profile::{add_profile, has_profile, insert_profile, remove_profile, remove_profile_entry},
};

#[test]
fn test_insert_profile() {
  let mut config = SetupConfig::new();
  insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
  assert!(config.profiles.contains_key("myprofile"));
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
fn test_add_profile_inserts_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = SetupConfig::new();
    config.path = Some(path.clone());

    let args = vec!["myprofile".to_string(), "user/repo1".to_string()];
    add_profile(&mut config, &args, true, io, make_flags()).unwrap();
    assert!(has_profile(&config, "myprofile"));
    assert!(path.exists());
  });
}

#[test]
fn test_add_profile_errors_on_insufficient_args() {
  let mut config = SetupConfig::new();
  let args = vec!["myprofile".to_string()];
  with_io_output(|io| {
    let result = add_profile(&mut config, &args, true, io, make_flags());
    assert!(result.is_err());
  });
}

#[test]
fn test_add_profile_errors_on_empty_args() {
  let mut config = SetupConfig::new();
  with_io_output(|io| {
    let result = add_profile(&mut config, &[], true, io, make_flags());
    assert!(result.is_err());
  });
}

#[test]
fn test_add_profile_overwrites_existing() {
  with_io_dir(|tmp, io| {
    let mut config = SetupConfig::new();
    config.path = Some(tmp.join(".star-setup.json"));
    insert_profile(&mut config, "myprofile", vec!["old/repo".to_string()]);

    let args = vec!["myprofile".to_string(), "new/repo".to_string()];
    add_profile(&mut config, &args, true, io, make_flags()).unwrap();
    assert_eq!(config.profiles["myprofile"], vec!["new/repo"]);
  });
}

#[test]
fn test_add_profile_multiple_repos() {
  with_io_dir(|tmp, io| {
    let mut config = SetupConfig::new();
    config.path = Some(tmp.join(".star-setup.json"));

    let args = vec![
      "myprofile".to_string(),
      "user/repo1".to_string(),
      "user/repo2".to_string(),
      "user/repo3".to_string(),
    ];
    add_profile(&mut config, &args, true, io, make_flags()).unwrap();
    assert_eq!(config.profiles["myprofile"].len(), 3);
  });
}

#[test]
fn test_add_profile_aborts_when_exists_and_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut config = SetupConfig::new();
    config.path = Some(tmp.path().join(".star-setup.json"));
    insert_profile(&mut config, "myprofile", vec!["old/repo".to_string()]);

    let args = vec!["myprofile".to_string(), "new/repo".to_string()];
    add_profile(&mut config, &args, false, io, make_flags()).unwrap();
    assert_eq!(config.profiles["myprofile"], vec!["old/repo"]);
  });
}

#[test]
fn test_remove_profile_removes_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = SetupConfig::new();
    config.path = Some(path.clone());
    insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);
    save_config(&mut config).unwrap();

    remove_profile(&mut config, "myprofile", true, io, make_flags()).unwrap();
    assert!(!has_profile(&config, "myprofile"));
  });
}

#[test]
fn test_remove_profile_not_found() {
  let mut config = SetupConfig::new();
  with_io_output(|io| {
    remove_profile(&mut config, "nonexistent", true, io, make_flags()).unwrap();
  });
}

#[test]
fn test_remove_profile_aborts_when_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let mut config = SetupConfig::new();
    insert_profile(&mut config, "myprofile", vec!["user/repo1".to_string()]);

    remove_profile(&mut config, "myprofile", false, io, make_flags()).unwrap();
    assert!(has_profile(&config, "myprofile"));
  });
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

  with_io_output(|io| {
    let loaded = load_config(&[path], &mut io.output);
    assert!(loaded.profiles.contains_key("myprofile"));
    assert_eq!(
      loaded.profiles["myprofile"],
      vec!["user/repo1", "user/repo2"]
    );
  });
}
