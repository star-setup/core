use super::{
  common::{empty_input, make_io, sink},
  fixtures::sample_entry,
};
use star_setup::{
  cli::BuildType,
  config::{
    add_config, create_default_config, has_config, insert_config, list_configs, remove_config,
    remove_config_entry, save_config, ConfigEntry, SetupConfig,
  },
};

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
fn test_add_config_inserts_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());

  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  add_config(&mut config, "myconfig", sample_entry(), true, &mut io).unwrap();
  assert!(has_config(&config, "myconfig"));
  assert!(path.exists());
}

#[test]
fn test_add_config_aborts_when_exists_and_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_config(&mut config, "myconfig", sample_entry());

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  add_config(
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
      timing: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
    false,
    &mut io,
  )
  .unwrap();
  assert!(config.configs["myconfig"].ssh);
}

#[test]
fn test_insert_config() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  assert!(config.configs.contains_key("myconfig"));
  assert!(config.configs["myconfig"].ssh);
}

#[test]
fn test_remove_config_entry_exists() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  assert!(remove_config_entry(&mut config, "myconfig"));
  assert!(!config.configs.contains_key("myconfig"));
}

#[test]
fn test_create_default_config_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");

  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  create_default_config(path.clone(), true, &mut io).unwrap();
  assert!(path.exists());
}

#[test]
fn test_create_default_config_aborts_when_exists_and_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  std::fs::write(&path, "original").unwrap();

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  create_default_config(path.clone(), false, &mut io).unwrap();
  assert_eq!(std::fs::read_to_string(&path).unwrap(), "original");
}

#[test]
fn test_list_configs_empty() {
  let config = SetupConfig::new();
  let mut output = sink();
  list_configs(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("No configurations created"));
}

#[test]
fn test_list_configs_with_entries() {
  let mut config = SetupConfig::new();
  insert_config(&mut config, "myconfig", sample_entry());
  let mut output = sink();
  list_configs(&config, &mut output);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("myconfig"));
  assert!(out.contains("Configurations:"));
}

#[test]
fn test_remove_config_entry_missing() {
  let mut config = SetupConfig::new();
  assert!(!remove_config_entry(&mut config, "nonexistent"));
}

#[test]
fn test_remove_config_removes_and_saves() {
  let tmp = tempfile::TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  insert_config(&mut config, "myconfig", sample_entry());
  save_config(&mut config).unwrap();

  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  remove_config(&mut config, "myconfig", true, &mut io).unwrap();
  assert!(!has_config(&config, "myconfig"));
}

#[test]
fn test_remove_config_not_found() {
  let mut config = SetupConfig::new();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  remove_config(&mut config, "nonexistent", true, &mut io).unwrap();
}

#[test]
fn test_remove_config_aborts_when_not_confirmed() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut config = SetupConfig::new();
  config.path = Some(tmp.path().join(".star-setup.json"));
  insert_config(&mut config, "myconfig", sample_entry());

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  remove_config(&mut config, "myconfig", false, &mut io).unwrap();
  assert!(has_config(&config, "myconfig"));
}
