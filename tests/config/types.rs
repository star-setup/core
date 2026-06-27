use star_setup::{
  cli::{BuildFlags, BuildType, ConnectionFlags, DiagnosticFlags, MonoRepoFlags},
  config::{ConfigEntry},
};
use crate::common::default_resolved;

#[test]
fn test_from_flags_defaults() {
  let connection = ConnectionFlags { ssh: false, https: false, verbose: false, no_verbose: false };
  let build = BuildFlags {
    build_type: None,
    build_dir: None,
    build_system: None,
    no_build: false,
    build: false,
    clean: false,
    no_clean: false,
    cmake_flags: vec![],
    meson_flags: vec![],
  };
  let mono = MonoRepoFlags { mono_repo: false, mono_dir: None, repos: None, profile: None };
  let diagnostic = DiagnosticFlags { timing: false, dry_run: false };

  let entry = ConfigEntry::from_flags(&connection, &build, &mono, &diagnostic);

  assert!(!entry.ssh);
  assert_eq!(entry.build_type, BuildType::Debug);
  assert_eq!(entry.build_dir, "build");
  assert_eq!(entry.mono_dir, "build-mono");
  assert!(!entry.no_build);
  assert!(!entry.clean);
  assert!(!entry.verbose);
  assert!(!entry.timing);
  assert!(!entry.dry_run);
}

#[test]
fn test_from_flags_with_values() {
  let connection = ConnectionFlags { ssh: true, https: false, verbose: true, no_verbose: false };
  let build = BuildFlags {
    build_type: Some("release".to_string()),
    build_dir: Some("out".to_string()),
    build_system: None,
    no_build: true,
    build: false,
    clean: true,
    no_clean: false,
    cmake_flags: vec!["-DFOO=ON".to_string()],
    meson_flags: vec![],
  };
  let mono = MonoRepoFlags { mono_repo: false, mono_dir: Some("workspace".to_string()), repos: None, profile: None };
  let diagnostic = DiagnosticFlags { timing: true, dry_run: true };

  let entry = ConfigEntry::from_flags(&connection, &build, &mono, &diagnostic);

  assert!(entry.ssh);
  assert_eq!(entry.build_type, BuildType::Release);
  assert_eq!(entry.build_dir, "out");
  assert_eq!(entry.mono_dir, "workspace");
  assert!(entry.no_build);
  assert!(entry.clean);
  assert!(entry.verbose);
  assert!(entry.timing);
  assert!(entry.dry_run);
  assert_eq!(entry.cmake_flags, vec!["-DFOO=ON"]);
}

#[test]
fn test_from_resolved_args() {
  let args = default_resolved();
  let entry = ConfigEntry::from(&args);

  assert!(!entry.ssh);
  assert_eq!(entry.build_type, BuildType::Debug);
  assert_eq!(entry.build_dir, "build");
  assert!(entry.no_build);
}
