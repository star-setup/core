use crate::cli::{BuildType, ConfigFlags, ProfileFlags};

/// Resolved connection flags after applying config and CLI overrides.
pub struct ResolvedConnectionFlags {
  pub ssh: bool,
  pub verbose: bool,
}

/// Resolved diagnostic flags after applying config and CLI overrides.
pub struct ResolvedDiagnosticFlags {
  pub timing: bool,
}

/// Resolved build flags after applying config and CLI overrides.
pub struct ResolvedBuildFlags {
  pub build_type: BuildType,
  pub build_dir: String,
  pub no_build: bool,
  pub clean: bool,
  pub cmake_flags: Vec<String>,
  pub meson_flags: Vec<String>,
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
  pub config: ConfigFlags,
  pub profile: ProfileFlags,
}
