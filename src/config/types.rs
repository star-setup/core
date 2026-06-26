use crate::cli::{BuildType, ResolvedArgs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Represents a single named configuration entry.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Default)]
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
  /// Additional `CMake` arguments.
  pub cmake_flags: Vec<String>,
  /// Additional `Meson` arguments.
  pub meson_flags: Vec<String>,
}

impl From<&ResolvedArgs> for ConfigEntry {
  fn from(args: &ResolvedArgs) -> Self {
    Self {
      ssh: args.connection.ssh,
      build_type: args.build.build_type.clone(),
      build_dir: args.build.build_dir.clone(),
      mono_dir: args.mono.mono_dir.clone(),
      no_build: args.build.no_build,
      clean: args.build.clean,
      verbose: args.connection.verbose,
      timing: args.diagnostic.timing,
      cmake_flags: args.build.cmake_flags.clone(),
      meson_flags: args.build.meson_flags.clone(),
    }
  }
}

/// Top-level configuration structure.
#[derive(Serialize, Deserialize, Default)]
pub struct SetupConfig {
  /// Named configuration entries.
  #[serde(default)]
  pub configs: HashMap<String, ConfigEntry>,
  /// Named profile entries mapping profile names to repository lists.
  #[serde(default)]
  pub profiles: HashMap<String, Vec<String>>,
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
