use star_setup::cli::flags::DiagnosticFlags;
use star_setup::cli::{
  build::BuildType, resolve_bool, resolve_with_config, Args, BuildFlags, ConfigFlags,
  ConnectionFlags, MonoRepoFlags, ProfileFlags,
};
use star_setup::config::types::{ConfigEntry, SetupConfig};

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

fn default_args() -> Args {
  Args {
    repo: None,
    yes: false,
    diagnostic: DiagnosticFlags { timing: false },
    connection: ConnectionFlags {
      ssh: false,
      https: false,
      verbose: false,
      no_verbose: false,
    },
    build: BuildFlags {
      build_type: None,
      build_dir: None,
      no_build: false,
      build: false,
      clean: false,
      no_clean: false,
      cmake_flags: vec![],
      meson_flags: vec![],
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
  assert_eq!(resolved.build.build_type, BuildType::Debug);
  assert_eq!(resolved.build.build_dir, "build");
  assert_eq!(resolved.mono.mono_dir, "build-mono");
  assert!(!resolved.build.no_build);
  assert!(!resolved.build.clean);
}

#[test]
fn test_resolve_with_config_applies_config_defaults() {
  let mut config = SetupConfig::new();
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: true,
      verbose: true,
      build_type: BuildType::Release,
      build_dir: "out".to_string(),
      mono_dir: "mono".to_string(),
      no_build: true,
      clean: true,
      cmake_flags: vec!["-DTEST=ON".to_string()],
      meson_flags: vec![],
    },
  );
  let resolved = resolve_with_config(default_args(), &config).unwrap();
  assert!(resolved.connection.ssh);
  assert!(resolved.connection.verbose);
  assert_eq!(resolved.build.build_type, BuildType::Release);
  assert_eq!(resolved.build.build_dir, "out");
  assert!(resolved.build.no_build);
  assert!(resolved.build.clean);
  assert_eq!(resolved.build.cmake_flags, vec!["-DTEST=ON"]);
}

#[test]
fn test_resolve_with_config_cli_overrides_config() {
  let mut config = SetupConfig::new();
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: false,
      verbose: false,
      build_type: BuildType::Debug,
      build_dir: "build".to_string(),
      mono_dir: "build-mono".to_string(),
      no_build: false,
      clean: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
  );
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

#[test]
fn test_resolve_with_config_named_config_pulls_correct_values() {
  let mut config = SetupConfig::new();
  config.configs.insert(
    "myconfig".to_string(),
    ConfigEntry {
      ssh: true,
      verbose: false,
      build_type: BuildType::RelWithDebInfo,
      build_dir: "out".to_string(),
      mono_dir: "mono".to_string(),
      no_build: false,
      clean: true,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
  );
  let mut args = default_args();
  args.config.config_name = Some("myconfig".to_string());
  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(resolved.connection.ssh);
  assert_eq!(resolved.build.build_type, BuildType::RelWithDebInfo);
  assert_eq!(resolved.build.build_dir, "out");
  assert!(resolved.build.clean);
}

#[test]
fn test_resolve_with_config_cli_cmake_flags_not_overwritten_by_config() {
  let mut config = SetupConfig::new();
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: false,
      verbose: false,
      build_type: BuildType::Debug,
      build_dir: "build".to_string(),
      mono_dir: "build-mono".to_string(),
      no_build: false,
      clean: false,
      cmake_flags: vec!["-DCONFIG_FLAG=ON".to_string()],
      meson_flags: vec![],
    },
  );
  let mut args = default_args();
  args.build.cmake_flags = vec!["-DCLI_FLAG=ON".to_string()];
  let resolved = resolve_with_config(args, &config).unwrap();
  assert_eq!(resolved.build.cmake_flags, vec!["-DCLI_FLAG=ON"]);
}

#[test]
fn test_resolve_with_config_negative_flags_override_config() {
  let mut config = SetupConfig::new();
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: true,
      verbose: true,
      build_type: BuildType::Debug,
      build_dir: "build".to_string(),
      mono_dir: "build-mono".to_string(),
      no_build: true,
      clean: true,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
  );

  let mut args = default_args();
  args.connection.https = true; // negates ssh
  args.connection.no_verbose = true; // negates verbose
  args.build.build = true; // negates no_build
  args.build.no_clean = true; // negates clean

  let resolved = resolve_with_config(args, &config).unwrap();
  assert!(
    !resolved.connection.ssh,
    "https should override config ssh:true"
  );
  assert!(
    !resolved.connection.verbose,
    "no_verbose should override config verbose:true"
  );
  assert!(
    !resolved.build.no_build,
    "build should override config no_build:true"
  );
  assert!(
    !resolved.build.clean,
    "no_clean should override config clean:true"
  );
}
