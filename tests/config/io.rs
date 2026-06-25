use super::{common::sink, fixtures::sample_entry};
use star_setup::{
  cli::BuildType,
  config::{insert_config, load_config, save_config, ConfigEntry, SetupConfig},
};
use std::path::PathBuf;

#[test]
fn test_save_and_load_roundtrip() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");

  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: true,
      build_type: BuildType::Release,
      build_dir: "build".to_string(),
      mono_dir: "mono".to_string(),
      no_build: false,
      clean: false,
      verbose: false,
      timing: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
  );
  save_config(&mut config).unwrap();

  let loaded = load_config(&[path], &mut sink());
  assert!(loaded.configs.contains_key("default"));
  assert!(loaded.configs["default"].ssh);
  assert_eq!(loaded.configs["default"].build_type, BuildType::Release);
  assert_eq!(loaded.configs["default"].mono_dir, "mono");
  assert_eq!(loaded.configs["default"].cmake_flags, Vec::<String>::new());
}

#[test]
fn test_load_config_skips_missing_local_file() {
  let config = load_config(&[], &mut sink());
  assert!(config.configs.is_empty());
}

#[test]
fn test_load_config_handles_invalid_json() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  std::fs::write(&path, "{invalid json").unwrap();

  let config = load_config(&[path], &mut sink());
  assert!(config.configs.is_empty());
}

#[test]
fn test_load_config_skips_nonexistent_path() {
  let config = load_config(
    &[PathBuf::from("/nonexistent/path/.star-setup.json")],
    &mut sink(),
  );
  assert!(config.configs.is_empty());
}

#[test]
fn test_load_config_first_valid_wins() {
  let tmp1 = tempfile::TempDir::new().unwrap();
  let tmp2 = tempfile::TempDir::new().unwrap();
  let path1 = tmp1.path().join(".star-setup.json");
  let path2 = tmp2.path().join(".star-setup.json");

  let mut config1 = SetupConfig::new();
  config1.path = Some(path1.clone());
  insert_config(&mut config1, "first", sample_entry());
  save_config(&mut config1).unwrap();

  let mut config2 = SetupConfig::new();
  config2.path = Some(path2.clone());
  insert_config(&mut config2, "second", sample_entry());
  save_config(&mut config2).unwrap();

  let loaded = load_config(&[path1, path2], &mut sink());
  assert!(loaded.configs.contains_key("first"));
  assert!(!loaded.configs.contains_key("second"));
}

#[test]
fn test_load_config_falls_through_invalid_to_valid() {
  let tmp1 = tempfile::TempDir::new().unwrap();
  let tmp2 = tempfile::TempDir::new().unwrap();
  let path1 = tmp1.path().join(".star-setup.json");
  let path2 = tmp2.path().join(".star-setup.json");

  std::fs::write(&path1, "{invalid json").unwrap();

  let mut config2 = SetupConfig::new();
  config2.path = Some(path2.clone());
  insert_config(&mut config2, "second", sample_entry());
  save_config(&mut config2).unwrap();

  let loaded = load_config(&[path1, path2], &mut sink());
  assert!(loaded.configs.contains_key("second"));
}
