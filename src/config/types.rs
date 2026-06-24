use crate::cli::build::BuildType;
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
  /// Additional `CMake` arguments.
  pub cmake_flags: Vec<String>,
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
