use crate::cli::{BuildSystem, BuildType};

/// Resolved connection flags after applying config and CLI overrides.
pub struct ResolvedConnectionFlags {
  pub ssh: bool,
}

/// Resolved diagnostic flags after applying config and CLI overrides.
pub struct ResolvedDiagnosticFlags {
  pub verbose: bool,
  pub timing: bool,
  pub dry_run: bool,
}

/// Resolved build flags after applying config and CLI overrides.
#[allow(clippy::struct_excessive_bools)]
pub struct ResolvedBuildFlags {
  pub build_type: BuildType,
  pub build_dir: String,
  pub build_system: Option<BuildSystem>,
  pub no_build: bool,
  pub clean: bool,
  pub cmake_flags: Vec<String>,
  pub meson_flags: Vec<String>,
  pub watch: bool,
  pub no_watch: bool,
}

/// Resolved mono-repo flags after applying config and CLI overrides.
pub struct ResolvedMonoFlags {
  pub mono_repo: bool,
  pub mono_dir: String,
  pub repos: Option<Vec<String>>,
  pub profile: Option<String>,
}

/// Fully resolved arguments ready for command execution.
pub struct ResolvedArgs {
  pub repo: Option<String>,
  pub yes: bool,
  pub connection: ResolvedConnectionFlags,
  pub diagnostic: ResolvedDiagnosticFlags,
  pub build: ResolvedBuildFlags,
  pub mono: ResolvedMonoFlags,
}
