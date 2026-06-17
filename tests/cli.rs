use star_setup::cli::{
  Args, BuildFlags, ConfigFlags, ConnectionFlags, MonoRepoFlags, ProfileFlags,
  resolve_bool, resolve_with_config,
};
use star_setup::config::{ConfigEntry, SetupConfig};

// resolve_bool
#[test]
fn test_resolve_bool_negative_overrides_all() {
  assert!(!resolve_bool(true, true, Some(true), true));
}

#[test]
fn test_resolve_bool_positive_overrides_config_and_default() {
  assert!(resolve_bool(true, false, Some(false), false));
}

#[test]
fn test_resolve_bool_uses_config_when_no_flags() {
  assert!(resolve_bool(false, false, Some(true), false));
}

#[test]
fn test_resolve_bool_uses_default_when_no_flags_no_config() {
  assert!(resolve_bool(false, false, None, true));
}

#[test]
fn test_resolve_bool_default_false_when_nothing_set() {
  assert!(!resolve_bool(false, false, None, false));
}

fn default_args() -> Args {
  Args {
    repo: None,
    cmake_flags: vec![],
    yes: false,
    connection: ConnectionFlags { ssh: false, https: false, verbose: false, no_verbose: false },
    build: BuildFlags {
      build_type: None,
      build_dir: None,
      no_build: false,
      build: false,
      clean: false,
      no_clean: false,
    },
    mono: MonoRepoFlags {
      mono_repo: false,
      mono_dir: None,
      repos: None,
      profile: None,
    },
    config: ConfigFlags {
      init_config: false,
      config_name: None,
      config_add: None,
      config_remove: None,
      list_configs: false,
    },
    profile: ProfileFlags {
      profile_add: None,
      profile_remove: None,
      list_profiles: false,
    },
  }
}

// resolve_with_config tests
#[test]
fn test_resolve_with_config_defaults_when_no_config() {
  let config = SetupConfig::new();
  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(!resolved.connection.ssh);
  assert!(!resolved.connection.verbose);
  assert_eq!(resolved.build.build_type, "Debug");
  assert_eq!(resolved.build.build_dir, "build");
  assert_eq!(resolved.mono.mono_dir, "build-mono");
  assert!(!resolved.build.no_build);
  assert!(!resolved.build.clean);
}

#[test]
fn test_resolve_with_config_applies_config_defaults() {
  let mut config = SetupConfig::new();
  config.configs.insert("default".to_string(), ConfigEntry {
    ssh: true,
    verbose: true,
    build_type: "Release".to_string(),
    build_dir: "out".to_string(),
    mono_dir: "mono".to_string(),
    no_build: true,
    clean: true,
    cmake_flags: vec!["-DTEST=ON".to_string()],
  });
  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(resolved.connection.ssh);
  assert!(resolved.connection.verbose);
  assert_eq!(resolved.build.build_type, "Release");
  assert_eq!(resolved.build.build_dir, "out");
  assert!(resolved.build.no_build);
  assert!(resolved.build.clean);
  assert_eq!(resolved.cmake_flags, vec!["-DTEST=ON"]);
}

#[test]
fn test_resolve_with_config_cli_overrides_config() {
  let mut config = SetupConfig::new();
  config.configs.insert("default".to_string(), ConfigEntry {
    ssh: false,
    verbose: false,
    build_type: "Debug".to_string(),
    build_dir: "build".to_string(),
    mono_dir: "build-mono".to_string(),
    no_build: false,
    clean: false,
    cmake_flags: vec![],
  });
  let mut args = default_args();
  args.connection.ssh = true;
  args.build.build_type = Some("Release".to_string());
  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, "Release");
}

#[test]
fn test_resolve_with_config_errors_on_missing_config_name() {
  let config = SetupConfig::new();
  let mut args = default_args();
  args.config.config_name = Some("nonexistent".to_string());
  let result = resolve_with_config(args, &config);
  assert!(result.is_err());
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
