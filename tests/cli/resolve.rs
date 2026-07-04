use crate::common::default_args;
use star_setup::{
  cli::{resolve_bool, resolve_with_config, BuildType},
  config::{ConfigEntry, SetupConfig},
};

/// Helper to quickly build a `SetupConfig` with a populated profile entry.
fn config_with_entry(name: &str, entry: ConfigEntry) -> SetupConfig {
  let mut config = SetupConfig::new();
  config.configs.insert(name.to_string(), entry);
  config
}

/* =====     RESOLVE_BOOL     ===== */
#[test]
fn test_resolve_bool() {
  #[allow(clippy::struct_excessive_bools)]
  struct Case {
    flag_pos: bool,
    flag_neg: bool,
    config: Option<bool>,
    default: bool,
    expected: bool,
    name: &'static str,
  }

  let cases = [
    Case {
      flag_pos: true,
      flag_neg: true,
      config: Some(true),
      default: true,
      expected: false,
      name: "negative override all",
    },
    Case {
      flag_pos: true,
      flag_neg: false,
      config: Some(false),
      default: false,
      expected: true,
      name: "positive override config and default",
    },
    Case {
      flag_pos: false,
      flag_neg: false,
      config: Some(true),
      default: false,
      expected: true,
      name: "use config when no flags",
    },
    Case {
      flag_pos: false,
      flag_neg: false,
      config: None,
      default: true,
      expected: true,
      name: "use default when no flags/config",
    },
    Case {
      flag_pos: false,
      flag_neg: false,
      config: None,
      default: false,
      expected: false,
      name: "default false when nothing set",
    },
  ];

  for c in cases {
    assert_eq!(
      resolve_bool(c.flag_pos, c.flag_neg, c.config, c.default),
      c.expected,
      "Failed test: {}",
      c.name
    );
  }
}

/* =====     RESOLVE_WITH_CONFIG     ===== */
#[test]
fn test_resolve_with_config_defaults_when_no_config() {
  let config = SetupConfig::new();
  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(!resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, BuildType::Debug);
  assert_eq!(resolved.build.build_dir, "build");
  assert_eq!(resolved.mono.mono_dir, "build-mono");
  assert!(!resolved.build.no_build);
  assert!(!resolved.build.clean);
}

#[test]
fn test_resolve_with_config_applies_config_defaults() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      ssh: true,
      verbose: true,
      build_type: BuildType::Release,
      build_dir: "out".to_string(),
      no_build: true,
      clean: true,
      cmake_flags: vec!["-DTEST=ON".to_string()],
      ..ConfigEntry::default()
    },
  );

  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, BuildType::Release);
  assert_eq!(resolved.build.build_dir, "out");
  assert!(resolved.build.no_build);
  assert!(resolved.build.clean);
  assert_eq!(resolved.build.cmake_flags, vec!["-DTEST=ON"]);
}

#[test]
fn test_resolve_with_config_cli_overrides_config() {
  let config = config_with_entry("default", ConfigEntry::default());

  let mut args = default_args();
  args.connection.ssh = true;
  args.build.build_type = Some("Release".to_string());

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, BuildType::Release);
}

#[test]
fn test_resolve_with_config_errors_on_missing_config_name() {
  let config = SetupConfig::new();
  let mut args = default_args();
  args.config_name = Some("nonexistent".to_string());

  assert!(resolve_with_config(args, &config).is_err());
}

#[test]
fn test_resolve_with_config_mono_repo_from_repos() {
  let config = SetupConfig::new();
  let mut args = default_args();
  args.mono.repos = Some(vec!["user/lib1".to_string()]);

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.mono.mono_repo);
}

#[test]
fn test_resolve_with_config_mono_repo_from_profile() {
  let config = SetupConfig::new();
  let mut args = default_args();
  args.mono.profile = Some("myprofile".to_string());

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.mono.mono_repo);
}

#[test]
fn test_resolve_with_config_named_config_pulls_correct_values() {
  let config = config_with_entry(
    "myconfig",
    ConfigEntry {
      ssh: true,
      build_type: BuildType::RelWithDebInfo,
      build_dir: "out".to_string(),
      clean: true,
      ..ConfigEntry::default()
    },
  );

  let mut args = default_args();
  args.config_name = Some("myconfig".to_string());

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, BuildType::RelWithDebInfo);
  assert_eq!(resolved.build.build_dir, "out");
  assert!(resolved.build.clean);
}

#[test]
fn test_resolve_with_config_cli_cmake_flags_not_overwritten_by_config() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      cmake_flags: vec!["-DCONFIG_FLAG=ON".to_string()],
      ..ConfigEntry::default()
    },
  );

  let mut args = default_args();
  args.build.cmake_flags = vec!["-DCLI_FLAG=ON".to_string()];

  let resolved = resolve_with_config(args, &config).unwrap();
  assert_eq!(resolved.build.cmake_flags, vec!["-DCLI_FLAG=ON"]);
}

#[test]
fn test_resolve_with_config_negative_flags_override_config() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      ssh: true,
      verbose: true,
      no_build: true,
      clean: true,
      ..ConfigEntry::default()
    },
  );

  let mut args = default_args();
  args.connection.https = true;
  args.diagnostic.no_verbose = true;
  args.build.build = true;
  args.build.no_clean = true;

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(!resolved.connection.ssh);
  assert!(!resolved.diagnostic.verbose);
  assert!(!resolved.build.no_build);
  assert!(!resolved.build.clean);
}

#[test]
fn test_resolve_with_config_watch_dev_defaults_from_config() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      watch: true,
      dev: true,
      ..ConfigEntry::default()
    },
  );

  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(resolved.build.watch);
  assert!(resolved.build.dev);
  assert!(!resolved.build.no_watch);
  assert!(!resolved.build.no_dev);
}

#[test]
fn test_resolve_with_config_cli_negatives_override_watch_dev_config() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      watch: true,
      dev: true,
      ..ConfigEntry::default()
    },
  );

  let mut args = default_args();
  args.build.no_watch = true;
  args.build.no_dev = true;

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(!resolved.build.watch);
  assert!(!resolved.build.dev);
  assert!(resolved.build.no_watch);
  assert!(resolved.build.no_dev);
}

#[test]
fn test_resolve_with_config_cli_positives_override_no_watch_no_dev_config() {
  let config = config_with_entry(
    "default",
    ConfigEntry {
      no_watch: true,
      no_dev: true,
      ..ConfigEntry::default()
    },
  );

  let mut args = default_args();
  args.build.watch = true;
  args.build.dev = true;

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.build.watch);
  assert!(resolved.build.dev);
  assert!(!resolved.build.no_watch);
  assert!(!resolved.build.no_dev);
}
