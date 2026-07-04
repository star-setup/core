use crate::common::with_io_output;
use star_setup::{
  cli::BuildType,
  config::{insert_config, load_config, save_config, ConfigEntry, SetupConfig},
};
use std::{fs::write, path::PathBuf};
use tempfile::TempDir;

/* =====     SAVE_CONFIG     ===== */
#[test]
fn test_save_and_load_roundtrip() {
  let tmp = TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");

  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: true,
      build_type: BuildType::Release,
      mono_dir: "mono".to_string(),
      ..ConfigEntry::default()
    },
  );

  with_io_output(|io| {
    save_config(&mut config, false, &mut io.output).unwrap();
    let loaded = load_config(&[path], false, false, &mut io.output);
    assert!(loaded.configs.contains_key("default"));
    assert!(loaded.configs["default"].ssh);
    assert_eq!(loaded.configs["default"].build_type, BuildType::Release);
    assert_eq!(loaded.configs["default"].mono_dir, "mono");
    assert_eq!(loaded.configs["default"].cmake_flags, Vec::<String>::new());
  });
}

/* =====     LOAD_CONFIG     ===== */
#[test]
fn test_load_config_skips_missing_local_file() {
  with_io_output(|io| {
    let config = load_config(&[], false, false, &mut io.output);
    assert!(config.configs.is_empty());
  });
}

#[test]
fn test_load_config_handles_invalid_json() {
  let tmp = TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  write(&path, "{invalid json").unwrap();

  with_io_output(|io| {
    let config = load_config(&[path], false, false, &mut io.output);
    assert!(config.configs.is_empty());
  });
}

#[test]
fn test_load_config_skips_nonexistent_path() {
  with_io_output(|io| {
    let config = load_config(
      &[PathBuf::from("/nonexistent/path/.star-setup.json")],
      false,
      false,
      &mut io.output,
    );
    assert!(config.configs.is_empty());
  });
}

#[test]
fn test_load_config_first_valid_wins() {
  let tmp1 = TempDir::new().unwrap();
  let tmp2 = TempDir::new().unwrap();
  let path1 = tmp1.path().join(".star-setup.json");
  let path2 = tmp2.path().join(".star-setup.json");

  let mut config1 = SetupConfig::new();
  let mut config2 = SetupConfig::new();

  with_io_output(|io| {
    config1.path = Some(path1.clone());
    insert_config(&mut config1, "first", ConfigEntry::default());
    save_config(&mut config1, false, &mut io.output).unwrap();

    config2.path = Some(path2.clone());
    insert_config(&mut config2, "second", ConfigEntry::default());
    save_config(&mut config2, false, &mut io.output).unwrap();

    let loaded = load_config(&[path1, path2], false, false, &mut io.output);
    assert!(loaded.configs.contains_key("first"));
    assert!(!loaded.configs.contains_key("second"));
  });
}

#[test]
fn test_load_config_falls_through_invalid_to_valid() {
  let tmp1 = TempDir::new().unwrap();
  let tmp2 = TempDir::new().unwrap();
  let path1 = tmp1.path().join(".star-setup.json");
  let path2 = tmp2.path().join(".star-setup.json");

  write(&path1, "{invalid json").unwrap();

  let mut config2 = SetupConfig::new();
  config2.path = Some(path2.clone());
  insert_config(&mut config2, "second", ConfigEntry::default());

  with_io_output(|io| {
    save_config(&mut config2, false, &mut io.output).unwrap();
    let loaded = load_config(&[path1, path2], false, false, &mut io.output);
    assert!(loaded.configs.contains_key("second"));
  });
}
