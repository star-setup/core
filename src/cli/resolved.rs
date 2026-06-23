use crate::cli::flags::{ConfigFlags, ProfileFlags};

/// Resolved connection flags after applying config and CLI overrides.
pub struct ResolvedConnectionFlags {
  pub ssh: bool,
  pub verbose: bool,
}

/// Resolved build flags after applying config and CLI overrides.
pub struct ResolvedBuildFlags {
  pub build_type: String,
  pub build_dir: String,
  pub no_build: bool,
  pub clean: bool,
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
  pub cmake_flags: Vec<String>,
  pub yes: bool,
  pub connection: ResolvedConnectionFlags,
  pub build: ResolvedBuildFlags,
  pub mono: ResolvedMonoFlags,
  pub config: ConfigFlags,
  pub profile: ProfileFlags,
}
