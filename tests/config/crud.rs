use super::fixtures::sample_entry;
use crate::common::{make_flags, with_io_dir, with_io_input_output, with_io_output};
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
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = SetupConfig::new();
    config.path = Some(path.clone());

    add_config(
      &mut config,
      "myconfig",
      sample_entry(),
      true,
      io,
      make_flags(),
    )
    .unwrap();
    assert!(has_config(&config, "myconfig"));
    assert!(path.exists());
  });
}

#[test]
fn test_add_config_aborts_when_exists_and_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut config = SetupConfig::new();
    config.path = Some(tmp.path().join(".star-setup.json"));
    insert_config(&mut config, "myconfig", sample_entry());

    add_config(
      &mut config,
      "myconfig",
      ConfigEntry {
        ssh: false,
        build_type: BuildType::Debug,
        build_dir: "build".to_string(),
        mono_dir: "mono".to_string(),
        no_build: false,
        clean: false,
        verbose: false,
        timing: false,
        dry_run: false,
        cmake_flags: vec![],
        meson_flags: vec![],
      },
      false,
      io,
      make_flags(),
    )
    .unwrap();
    assert!(config.configs["myconfig"].ssh);
  });
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
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    create_default_config(path.clone(), true, io, make_flags()).unwrap();
    assert!(path.exists());
  });
}

#[test]
fn test_create_default_config_aborts_when_exists_and_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = tempfile::TempDir::new().unwrap();
    let path = tmp.path().join(".star-setup.json");
    std::fs::write(&path, "original").unwrap();

    create_default_config(path.clone(), false, io, make_flags()).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "original");
  });
}

#[test]
fn test_list_configs_empty() {
  let ((), out) = with_io_output(|io| {
    let config = SetupConfig::new();
    list_configs(&config, io);
  });
  assert!(out.contains("No configurations created"));
}

#[test]
fn test_list_configs_with_entries() {
  let ((), out) = with_io_output(|io| {
    let mut config = SetupConfig::new();
    insert_config(&mut config, "myconfig", sample_entry());
    list_configs(&config, io);
  });
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
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = SetupConfig::new();
    config.path = Some(path.clone());
    insert_config(&mut config, "myconfig", sample_entry());
    save_config(&mut config).unwrap();

    remove_config(&mut config, "myconfig", true, io, make_flags()).unwrap();
    assert!(!has_config(&config, "myconfig"));
  });
}

#[test]
fn test_remove_config_not_found() {
  let mut config = SetupConfig::new();
  with_io_output(|io| {
    remove_config(&mut config, "nonexistent", true, io, make_flags()).unwrap();
  });
}

#[test]
fn test_remove_config_aborts_when_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = tempfile::TempDir::new().unwrap();
    let mut config = SetupConfig::new();
    config.path = Some(tmp.path().join(".star-setup.json"));
    insert_config(&mut config, "myconfig", sample_entry());

    remove_config(&mut config, "myconfig", false, io, make_flags()).unwrap();
    assert!(has_config(&config, "myconfig"));
  });
}
