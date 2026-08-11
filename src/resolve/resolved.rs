use crate::{
  build::{BuildSystem, BuildType},
  ctx::RunFlags,
};

/// Resolved connection flags after applying config and CLI overrides.
#[derive(Debug)]
pub struct ResolvedConnectionFlags {
  pub ssh: bool,
}

/// Resolved build flags after applying config and CLI overrides.
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct ResolvedBuildFlags {
  pub build_type: BuildType,
  pub build_dir: String,
  pub build_system: Option<BuildSystem>,
  pub no_build: bool,
  pub clean: bool,
  pub watch: bool,
  pub no_watch: bool,
  pub dev: bool,
  pub no_dev: bool,
  pub cmake_flags: Vec<String>,
  pub meson_flags: Vec<String>,
}

/// Resolved mono-repo flags after applying config and CLI overrides.
#[derive(Debug)]
pub struct ResolvedMonoFlags {
  pub mono_repo: bool,
  pub mono_dir: String,
  pub deps: Option<Vec<String>>,
  pub profile: Option<String>,
}

/// Fully resolved arguments ready for command execution.
#[derive(Debug)]
pub struct ResolvedArgs {
  pub repo: Option<String>,
  pub yes: bool,
  pub connection: ResolvedConnectionFlags,
  pub diagnostic: RunFlags,
  pub build: ResolvedBuildFlags,
  pub mono: ResolvedMonoFlags,
}
