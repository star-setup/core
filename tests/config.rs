use star_setup::{
  cli::BuildType,
  config::{
    format_entry, has_config, insert_config, load_config, remove_config_entry, save_config,
    ConfigEntry, SetupConfig,
  },
};
use std::path::PathBuf;
mod common;
use common::{empty_input, sink};

fn sample_entry() -> ConfigEntry {
  ConfigEntry {
    ssh: true,
    build_type: BuildType::Release,
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
  assert!(config.configs["myconfig"].ssh);
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
  assert!(output.contains("-DTEST=ON"));
  assert!(output.contains("-DDEBUG=OFF"));
}
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
      cmake_flags: vec![],
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
fn test_create_default_config_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");

  star_setup::config::create_default_config(path.clone(), true, &mut empty_input(), &mut sink())
    .unwrap();
  assert!(path.exists());
}

#[test]
fn test_add_config_inserts_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());

  star_setup::config::add_config(
    &mut config,
    "myconfig",
    sample_entry(),
    true,
    &mut empty_input(),
    &mut sink(),
  )
  .unwrap();
  assert!(has_config(&config, "myconfig"));
  assert!(path.exists());
}

#[test]
fn test_remove_config_removes_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_config(&mut config, "myconfig", sample_entry());
  save_config(&mut config).unwrap();

  star_setup::config::remove_config(
    &mut config,
    "myconfig",
    true,
    &mut empty_input(),
    &mut sink(),
  )
  .unwrap();
  assert!(!has_config(&config, "myconfig"));
}

#[test]
fn test_remove_config_not_found() {
  let mut config = SetupConfig::new();
  star_setup::config::remove_config(
    &mut config,
    "nonexistent",
    true,
    &mut empty_input(),
    &mut sink(),
  )
  .unwrap();
}

#[test]
fn test_add_config_aborts_when_exists_and_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_config(&mut config, "myconfig", sample_entry());

  let input = b"n\n";
  star_setup::config::add_config(
    &mut config,
    "myconfig",
    ConfigEntry {
      ssh: false, // different from sample_entry's ssh: true
      build_type: BuildType::Debug,
      build_dir: "build".to_string(),
      mono_dir: "mono".to_string(),
      no_build: false,
      clean: false,
      verbose: false,
      cmake_flags: vec![],
    },
    false,
    &mut input.as_ref(),
    &mut sink(),
  )
  .unwrap();
  assert!(config.configs["myconfig"].ssh);
}

#[test]
fn test_remove_config_aborts_when_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_config(&mut config, "myconfig", sample_entry());

  let input = b"n\n";
  star_setup::config::remove_config(
    &mut config,
    "myconfig",
    false,
    &mut input.as_ref(),
    &mut sink(),
  )
  .unwrap();
  assert!(has_config(&config, "myconfig"));
}

#[test]
fn test_create_default_config_aborts_when_exists_and_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  std::fs::write(&path, "original").unwrap();

  let input = b"n\n";
  star_setup::config::create_default_config(path.clone(), false, &mut input.as_ref(), &mut sink())
    .unwrap();
  assert_eq!(std::fs::read_to_string(&path).unwrap(), "original");
}

#[test]
fn test_list_configs_empty() {
  let config = SetupConfig::new();
  let mut output = sink();
  star_setup::config::list_configs(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("No configurations created"));
}

#[test]
fn test_list_configs_with_entries() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  let mut output = sink();
  star_setup::config::list_configs(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("myconfig"));
  assert!(out.contains("Configurations:"));
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
