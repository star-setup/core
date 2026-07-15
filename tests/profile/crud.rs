use std::collections::HashMap;

use crate::common::{make_flags, with_io_dir, with_io_input_output, with_io_output};
use star_setup::{
  config::{load_config, save_config, Config},
  profile::{
    add_profile, has_profile, insert_profile, remove_profile, remove_profile_entry, Profile,
  },
};
use tempfile::TempDir;

/* =====     HAS_PROFILE     ===== */
#[test]
fn test_has_profile_false() {
  let config = Config::new();
  assert!(!has_profile(&config, "nonexistent"));
}

#[test]
fn test_add_profile_inserts_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = Config::new();
    config.path = Some(path.clone());

    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
        deps: HashMap::from([("default".to_string(), vec!["user/repo1".to_string()])]),
      },
      true,
      io,
      make_flags(),
    )
    .unwrap();

    assert!(has_profile(&config, "myprofile"));
    assert!(path.exists());
  });
}

/* =====     SAVE_CONFIG     ===== */
#[test]
fn test_save_and_load_profile_roundtrip() {
  let tmp = TempDir::new().unwrap();
  let path = tmp.path().join(".star-setup.json");
  let mut config = Config::new();
  config.path = Some(path.clone());

  insert_profile(
    &mut config,
    "myprofile",
    Profile {
      test_repos: HashMap::new(),
      deps: HashMap::from([("default".to_string(), vec!["user/repo1".to_string(), "user/repo2".to_string()])]),
    },
  );

  with_io_output(|io| {
    save_config(&mut config, false, &mut io.output).unwrap();
    let loaded = load_config(&[path], false, false, &mut io.output);
    assert!(loaded.profiles.contains_key("myprofile"));
    assert_eq!(
      loaded.profiles["myprofile"].deps["default"],
      vec!["user/repo1", "user/repo2"]
    );
  });
}

/* =====     INSERT_PROFILE     ===== */
#[test]
fn test_insert_profile() {
  let mut config = Config::new();

  insert_profile(
    &mut config,
    "myprofile",
    Profile {
      test_repos: HashMap::new(),
       deps: HashMap::from([("default".to_string(), vec!["user/repo1".to_string()])]),
    },
  );

  assert!(config.profiles.contains_key("myprofile"));
}

#[test]
fn test_remove_profile_entry_exists() {
  let mut config = Config::new();

  insert_profile(&mut config, "myprofile", Profile::default());

  assert!(remove_profile_entry(&mut config, "myprofile"));
  assert!(!config.profiles.contains_key("myprofile"));
}

#[test]
fn test_has_profile_true() {
  let mut config = Config::new();
  insert_profile(&mut config, "myprofile", Profile::default());
  assert!(has_profile(&config, "myprofile"));
}

/* =====     ADD_PROFILE     ===== */
#[test]
fn test_add_profile_overwrites_existing() {
  with_io_dir(|tmp, io| {
    let mut config = Config::new();
    config.path = Some(tmp.join(".star-setup.json"));

    insert_profile(
      &mut config,
      "myprofile",
      Profile {
        test_repos: HashMap::new(),
        deps: HashMap::from([("default".to_string(), vec!["old/repo".to_string()])]),
      },
    );
    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
        deps: HashMap::from([("default".to_string(), vec!["new/repo".to_string()])]),
      },
      true,
      io,
      make_flags(),
    )
    .unwrap();
    assert_eq!( config.profiles["myprofile"].deps["default"], vec!["new/repo"]);
  });
}

#[test]
fn test_add_profile_multiple_repos() {
  with_io_dir(|tmp, io| {
    let mut config = Config::new();
    config.path = Some(tmp.join(".star-setup.json"));

    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
         deps: HashMap::from([("default".to_string(), vec![
          "user/repo1".to_string(),
          "user/repo2".to_string(),
          "user/repo3".to_string(),
        ])]),
      },
      true,
      io,
      make_flags(),
    )
    .unwrap();

    assert_eq!(config.profiles["myprofile"].deps["default"].len(), 3);
  });
}

#[test]
fn test_add_profile_aborts_when_exists_and_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let tmp = TempDir::new().unwrap();
    let mut config = Config::new();
    config.path = Some(tmp.path().join(".star-setup.json"));

    insert_profile(
      &mut config,
      "myprofile",
      Profile {
        test_repos: HashMap::new(),
         deps: HashMap::from([("default".to_string(), vec!["old/repo".to_string()])]),
      },
    );
    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
         deps: HashMap::from([("default".to_string(), vec!["new/repo".to_string()])]),
      },
      false,
      io,
      make_flags(),
    )
    .unwrap();
    assert_eq!(config.profiles["myprofile"].deps["default"], vec!["old/repo"]);
  });
}

/* =====     REMOVE_PROFILE_ENTRY     ===== */
#[test]
fn test_remove_profile_entry_missing() {
  let mut config = Config::new();
  assert!(!remove_profile_entry(&mut config, "nonexistent"));
}

/* =====     REMOVE_PROFILE     ===== */
#[test]
fn test_remove_profile_removes_and_saves() {
  with_io_dir(|tmp, io| {
    let path = tmp.join(".star-setup.json");
    let mut config = Config::new();
    config.path = Some(path.clone());

    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
        deps: HashMap::from([("default".to_string(), vec!["user/repo1".to_string()])]),
      },
      true,
      io,
      make_flags(),
    )
    .unwrap();

    save_config(&mut config, false, &mut io.output).unwrap();

    remove_profile(&mut config, "myprofile", true, io, make_flags()).unwrap();

    assert!(!has_profile(&config, "myprofile"));
  });
}

#[test]
fn test_remove_profile_not_found() {
  let mut config = Config::new();
  with_io_output(|io| {
    remove_profile(&mut config, "nonexistent", true, io, make_flags()).unwrap();
  });
}

#[test]
fn test_remove_profile_aborts_when_not_confirmed() {
  with_io_input_output(b"n\n", |io| {
    let mut config = Config::new();

    add_profile(
      &mut config,
      "myprofile",
      &Profile {
        test_repos: HashMap::new(),
        deps: HashMap::from([("default".to_string(), vec!["user/repo1".to_string()])]),
      },
      true,
      io,
      make_flags(),
    )
    .unwrap();

    remove_profile(&mut config, "myprofile", false, io, make_flags()).unwrap();

    assert!(has_profile(&config, "myprofile"));
  });
}
