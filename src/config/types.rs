use crate::cli::{
  BuildFlags, BuildType, ConnectionFlags, DiagnosticFlags, MonoRepoFlags, ResolvedArgs,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Represents a single named configuration entry.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigEntry {
  /// Use SSH instead of HTTPS for cloning.
  pub ssh: bool,
  /// Build type (e.g. `Debug`, `Release`).
  pub build_type: BuildType,
  /// Build directory name.
  pub build_dir: String,
  /// Mono-repo build directory name.
  pub mono_dir: String,
  /// Skip the build step, only configure.
  pub no_build: bool,
  /// Clean the build directory before configuring.
  pub clean: bool,
  /// Show detailed command output.
  pub verbose: bool,
  /// Show timing information.
  pub timing: bool,
  /// Print commands instead of executing them.
  pub dry_run: bool,
  /// Automatically open the libraries watch scripts.
  pub watch: bool,
  /// Do not generate the libraries watch scripts.
  pub no_watch: bool,
  /// Automatically open the test repo's dev server.
  pub dev: bool,
  /// Do not open the test repo's dev server.
  pub no_dev: bool,
  /// Additional `CMake` arguments.
  pub cmake_flags: Vec<String>,
  /// Additional `Meson` arguments.
  pub meson_flags: Vec<String>,
}

impl ConfigEntry {
  /// Creates a `ConfigEntry` from raw flag structs.
  #[must_use]
  pub fn from_flags(
    connection: &ConnectionFlags,
    build: &BuildFlags,
    mono: &MonoRepoFlags,
    diagnostic: &DiagnosticFlags,
  ) -> Self {
    Self {
      ssh: connection.ssh,
      build_type: build
        .build_type
        .as_deref()
        .unwrap_or("debug")
        .parse()
        .unwrap_or_default(),
      build_dir: build
        .build_dir
        .clone()
        .unwrap_or_else(|| "build".to_string()),
      mono_dir: mono
        .mono_dir
        .clone()
        .unwrap_or_else(|| "build-mono".to_string()),
      no_build: build.no_build,
      clean: build.clean,
      verbose: diagnostic.verbose,
      timing: diagnostic.timing,
      dry_run: diagnostic.dry_run,
      watch: build.watch,
      no_watch: build.no_watch,
      dev: build.dev,
      no_dev: build.no_dev,
      cmake_flags: build.cmake_flags.clone(),
      meson_flags: build.meson_flags.clone(),
    }
  }
}

impl Default for ConfigEntry {
  fn default() -> Self {
    Self {
      ssh: false,
      build_type: BuildType::Debug,
      build_dir: "build".to_string(),
      mono_dir: "build-mono".to_string(),
      no_build: false,
      clean: false,
      verbose: false,
      timing: false,
      dry_run: false,
      watch: false,
      no_watch: false,
      dev: false,
      no_dev: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    }
  }
}

impl From<&ResolvedArgs> for ConfigEntry {
  fn from(args: &ResolvedArgs) -> Self {
    Self {
      ssh: args.connection.ssh,
      build_type: args.build.build_type,
      build_dir: args.build.build_dir.clone(),
      mono_dir: args.mono.mono_dir.clone(),
      no_build: args.build.no_build,
      clean: args.build.clean,
      verbose: args.diagnostic.verbose,
      timing: args.diagnostic.timing,
      dry_run: args.diagnostic.dry_run,
      watch: args.build.watch,
      no_watch: args.build.no_watch,
      dev: args.build.dev,
      no_dev: args.build.no_dev,
      cmake_flags: args.build.cmake_flags.clone(),
      meson_flags: args.build.meson_flags.clone(),
    }
  }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Profile {
  pub test_repo: Option<String>,
  pub deps: Vec<String>,
}

/// Top-level configuration structure.
#[derive(Serialize, Deserialize, Default)]
pub struct SetupConfig {
  /// Named configuration entries.
  #[serde(default)]
  pub configs: HashMap<String, ConfigEntry>,
  /// Named profile entries mapping profile names to repository lists.
  #[serde(default)]
  pub profiles: HashMap<String, Profile>,
  /// Path to the config file this was loaded from, if any.
  #[serde(skip)]
  pub path: Option<PathBuf>,
}

impl SetupConfig {
  /// Creates a new empty `SetupConfig`.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }
}
