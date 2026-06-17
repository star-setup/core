use star_setup::config::{
  format_entry, has_config, insert_config, load_config, remove_config_entry, save_config,
  ConfigEntry, SetupConfig,
};
use std::path::PathBuf;

fn sample_entry() -> ConfigEntry {
  ConfigEntry {
    ssh: true,
    build_type: "Release".to_string(),
    build_dir: "build".to_string(),
    mono_dir: "mono".to_string(),
    no_build: false,
    clean: true,
    verbose: false,
    cmake_flags: vec![],
  }
}

#[test]
fn test_insert_config() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  assert!(config.configs.contains_key("myconfig"));
  assert_eq!(config.configs["myconfig"].ssh, true);
}

#[test]
fn test_has_config_true() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  assert!(has_config(&config, "myconfig"));
}

#[test]
fn test_has_config_false() {
  let config = SetupConfig::new();
  assert!(!has_config(&config, "nonexistent"));
}

#[test]
fn test_remove_config_entry_exists() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  assert!(remove_config_entry(&mut config, "myconfig"));
  assert!(!config.configs.contains_key("myconfig"));
}

#[test]
fn test_remove_config_entry_missing() {
  let mut config = SetupConfig::new();
  assert!(!remove_config_entry(&mut config, "nonexistent"));
}

#[test]
fn test_format_entry_contains_fields() {
  let entry = sample_entry();
  let output = format_entry(&entry);
  assert!(output.contains("SSH: true"));
  assert!(output.contains("Build Type: Release"));
  assert!(output.contains("Clean flag: true"));
}

#[test]
fn test_format_entry_single_cmake_flag() {
  let mut entry = sample_entry();
  entry.cmake_flags = vec!["-DTEST=ON".to_string()];
  let output = format_entry(&entry);
  assert!(output.contains("CMake argument: -DTEST=ON"));
}

#[test]
fn test_format_entry_multiple_cmake_flags() {
  let mut entry = sample_entry();
  entry.cmake_flags = vec!["-DTEST=ON".to_string(), "-DDEBUG=OFF".to_string()];
  let output = format_entry(&entry);
  assert!(output.contains("CMake arguments:"));
}
#[test]
fn test_save_and_load_roundtrip() {
  let tmp = std::env::temp_dir().join("star_setup_test_roundtrip");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");

  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: true,
      build_type: "Release".to_string(),
      build_dir: "build".to_string(),
      mono_dir: "mono".to_string(),
      no_build: false,
      clean: false,
      verbose: false,
      cmake_flags: vec![],
    },
  );
  save_config(&mut config).unwrap();

  let loaded = load_config(&[path]);
  assert!(loaded.configs.contains_key("default"));
  assert_eq!(loaded.configs["default"].ssh, true);

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_load_config_skips_missing_local_file() {
  let config = load_config(&[]);
  assert!(config.configs.is_empty());
}

#[test]
fn test_load_config_handles_invalid_json() {
  let tmp = std::env::temp_dir().join("star_setup_test_invalid_json");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");
  std::fs::write(&path, "{invalid json").unwrap();

  let config = load_config(&[path]);
  assert!(config.configs.is_empty());

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_load_config_skips_nonexistent_path() {
  let config = load_config(&[PathBuf::from("/nonexistent/path/.star-setup.json")]);
  assert!(config.configs.is_empty());
}

#[test]
fn test_create_default_config_creates_file() {
  let tmp = std::env::temp_dir().join("star_setup_test_create_default");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");

  star_setup::config::create_default_config(path.clone(), true).unwrap();
  assert!(path.exists());

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_add_config_inserts_and_saves() {
  let tmp = std::env::temp_dir().join("star_setup_test_add_config");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());

  star_setup::config::add_config(&mut config, "myconfig", sample_entry(), true).unwrap();
  assert!(has_config(&config, "myconfig"));
  assert!(path.exists());

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_remove_config_removes_and_saves() {
  let tmp = std::env::temp_dir().join("star_setup_test_remove_config");
  std::fs::create_dir_all(&tmp).ok();
  let path = tmp.join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_config(&mut config, "myconfig", sample_entry());
  save_config(&mut config).unwrap();

  star_setup::config::remove_config(&mut config, "myconfig", true).unwrap();
  assert!(!has_config(&config, "myconfig"));

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_remove_config_not_found() {
  let mut config = SetupConfig::new();
  star_setup::config::remove_config(&mut config, "nonexistent", true).unwrap();
}
