use crate::common::{make_flags, with_io_dir, with_io_input_output, with_io_output};
use star_setup::config::{
  add_config, create_default_config, has_config, insert_config, list_configs, remove_config,
  remove_config_entry, save_config, Config, ConfigEntry,
};
use std::fs::{read_to_string, write};
use tempfile::TempDir;

/* =====     HAS_CONFIG     ===== */
#[test]
fn test_has_config_false() {
  let config = Config::new();
  assert!(!has_config(&config, "nonexistent"));
}

#[test]
fn test_add_config_inserts_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = Config::new();
    config.path = Some(path.clone());

    add_config(
      &mut config,
      "myconfig",
      ConfigEntry::default(),
      true,
      io,
      make_flags(),
    )
    .unwrap();
    assert!(has_config(&config, "myconfig"));
    assert!(path.exists());
  });
}

/* =====     ADD_CONFIG     ===== */
#[test]
fn test_add_config_aborts_when_exists_and_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = TempDir::new().unwrap();
    let mut config = Config::new();
    config.path = Some(tmp.path().join(".star-setup.json"));
    insert_config(
      &mut config,
      "myconfig",
      ConfigEntry {
        ssh: true,
        ..ConfigEntry::default()
      },
    );

    add_config(
      &mut config,
      "myconfig",
      ConfigEntry::default(),
      false,
      io,
      make_flags(),
    )
    .unwrap();
    assert!(config.configs["myconfig"].ssh);
  });
}

/* =====     INSERT_CONFIG     ===== */
#[test]
fn test_has_config_true() {
  let mut config = Config::new();
  insert_config(&mut config, "myconfig", ConfigEntry::default());
  assert!(has_config(&config, "myconfig"));
}

#[test]
fn test_insert_config() {
  let mut config = Config::new();
  insert_config(
    &mut config,
    "myconfig",
    ConfigEntry {
      ssh: true,
      ..ConfigEntry::default()
    },
  );
  assert!(config.configs.contains_key("myconfig"));
  assert!(config.configs["myconfig"].ssh);
}

#[test]
fn test_remove_config_entry_exists() {
  let mut config = Config::new();
  insert_config(&mut config, "myconfig", ConfigEntry::default());
  assert!(remove_config_entry(&mut config, "myconfig"));
  assert!(!config.configs.contains_key("myconfig"));
}

/* =====     CREATE_DEFAULT_CONFIG     ===== */
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
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join(".star-setup.json");
    write(&path, "original").unwrap();

    create_default_config(path.clone(), false, io, make_flags()).unwrap();
    assert_eq!(read_to_string(&path).unwrap(), "original");
  });
}

/* =====     LIST_CONFIGS     ===== */
#[test]
fn test_list_configs_empty() {
  let ((), out) = with_io_output(|io| {
    let config = Config::new();
    list_configs(&config, io);
  });
  assert!(out.contains("No configurations created"));
}

#[test]
fn test_list_configs_with_entries() {
  let ((), out) = with_io_output(|io| {
    let mut config = Config::new();
    insert_config(&mut config, "myconfig", ConfigEntry::default());
    list_configs(&config, io);
  });
  assert!(out.contains("myconfig"));
  assert!(out.contains("Configurations:"));
}

/* =====     REMOVE_CONFIG_ENTRY     ===== */
#[test]
fn test_remove_config_entry_missing() {
  let mut config = Config::new();
  assert!(!remove_config_entry(&mut config, "nonexistent"));
}

/* =====     REMOVE_CONFIG     ===== */
#[test]
fn test_remove_config_removes_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = Config::new();
    config.path = Some(path.clone());
    insert_config(&mut config, "myconfig", ConfigEntry::default());
    save_config(&mut config, false, &mut io.output).unwrap();

    remove_config(&mut config, "myconfig", true, io, make_flags()).unwrap();
    assert!(!has_config(&config, "myconfig"));
  });
}

#[test]
fn test_remove_config_not_found() {
  let mut config = Config::new();
  with_io_output(|io| {
    remove_config(&mut config, "nonexistent", true, io, make_flags()).unwrap();
  });
}

#[test]
fn test_remove_config_aborts_when_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = TempDir::new().unwrap();
    let mut config = Config::new();
    config.path = Some(tmp.path().join(".star-setup.json"));
    insert_config(&mut config, "myconfig", ConfigEntry::default());

    remove_config(&mut config, "myconfig", false, io, make_flags()).unwrap();
    assert!(has_config(&config, "myconfig"));
  });
}
