//! Configuration file management.

use crate::utils::confirm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::path::PathBuf;

/// Represents a single named configuration entry.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Default)]
pub struct ConfigEntry {
  /// Use SSH instead of HTTPS for cloning.
  pub ssh: bool,
  /// `CMake` build type (e.g. `Debug`, `Release`).
  pub build_type: String,
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

/// Inserts or overwrites a named configuration entry.
pub fn insert_config(config: &mut SetupConfig, name: &str, entry: ConfigEntry) {
  config.configs.insert(name.to_string(), entry);
}

/// Removes a named configuration entry. Returns `true` if it existed.
pub fn remove_config_entry(config: &mut SetupConfig, name: &str) -> bool {
  config.configs.remove(name).is_some()
}

/// Returns `true` if a configuration with the given name exists.
#[must_use]
pub fn has_config(config: &SetupConfig, name: &str) -> bool {
  config.configs.contains_key(name)
}

/// Formats a `ConfigEntry` as a human-readable string.
#[must_use]
pub fn format_entry(e: &ConfigEntry) -> String {
  let mut out = String::new();
  writeln!(out, "  SSH: {}", e.ssh).ok();
  writeln!(out, "  Build Type: {}", e.build_type).ok();
  writeln!(out, "  Build Directory: {}", e.build_dir).ok();
  writeln!(out, "  Mono-build Directory: {}", e.mono_dir).ok();
  writeln!(out, "  No-build flag: {}", e.no_build).ok();
  writeln!(out, "  Clean flag: {}", e.clean).ok();
  writeln!(out, "  Verbose flag: {}", e.verbose).ok();
  if e.cmake_flags.is_empty() {
    out.push('\n');
  } else if e.cmake_flags.len() == 1 {
    writeln!(out, "  CMake argument: {}", e.cmake_flags[0]).ok();
  } else {
    out.push_str("  CMake arguments:\n");
    for arg in &e.cmake_flags {
      writeln!(out, "    {arg}").ok();
    }
  }
  out
}

/// Loads configuration from the first valid JSON file in `locations`.
pub fn load_config(locations: &[PathBuf], output: &mut impl Write) -> SetupConfig {
  let mut invalid_count = 0;

  for path in locations {
    if !path.exists() {
      continue;
    }
    match fs::read_to_string(path) {
      Ok(contents) => match serde_json::from_str::<SetupConfig>(&contents) {
        Ok(mut config) => {
          config.path = Some(path.clone());
          return config;
        }
        Err(e) => {
          writeln!(output, "Warning: Invalid JSON in {}: {e}", path.display()).ok();
          invalid_count += 1;
        }
      },
      Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
        writeln!(output, "Error: No permission to read {}", path.display()).ok();
        invalid_count += 1;
      }
      Err(e) => {
        writeln!(
          output,
          "An unexpected error occurred reading {}: {e}",
          path.display()
        )
        .ok();
        invalid_count += 1;
      }
    }
  }

  if invalid_count != 0 {
    writeln!(
      output,
      "Found {invalid_count} config file{} that had errors",
      if invalid_count == 1 { "" } else { "s" }
    )
    .ok();
  }
  SetupConfig::new()
}

/// Serializes the configuration and writes it to the path stored in `config.path`.
/// # Errors
/// Returns an error if serialization fails or if the file cannot be written.
pub fn save_config(config: &mut SetupConfig) -> Result<PathBuf, String> {
  let path = config
    .path
    .get_or_insert_with(|| {
      dirs::home_dir().map_or_else(
        || PathBuf::from(".star-setup.json"),
        |h| h.join(".star-setup.json"),
      )
    })
    .clone();
  let json =
    serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {e}"))?;

  fs::write(&path, json).map_err(|e| match e.kind() {
    io::ErrorKind::PermissionDenied => {
      format!("Error: No permission to write to {}", path.display())
    }
    _ => format!(
      "An unexpected error occurred writing {}: {}",
      path.display(),
      e
    ),
  })?;
  Ok(path)
}

/// Creates a default configuration file in the current directory.
/// # Errors
/// Returns an error if the config file cannot be written.
pub fn create_default_config(
  path: PathBuf,
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  if path.exists()
    && !confirm(
      &format!("{} already exists. Overwrite?", path.display()),
      yes,
      input,
      output,
    )
  {
    writeln!(output, "Aborted.").ok();
    return Ok(());
  }

  let mut config = SetupConfig::new();
  config.path = Some(path.clone());
  config.configs.insert(
    "default".to_string(),
    ConfigEntry {
      ssh: false,
      build_type: "Debug".to_string(),
      build_dir: "build".to_string(),
      mono_dir: "build-mono".to_string(),
      no_build: false,
      clean: false,
      verbose: false,
      cmake_flags: vec![],
    },
  );

  save_config(&mut config)?;

  writeln!(
    output,
    "Created config file: {}",
    dunce::canonicalize(&path).unwrap_or(path).display()
  )
  .ok();
  writeln!(output, "Edit this file to customize your defaults.").ok();
  writeln!(output, "\nConfig files are checked in this order:").ok();
  writeln!(output, "  1. ./.star-setup.json (current directory)").ok();
  writeln!(output, "  2. ~/.star-setup.json (home directory)").ok();

  Ok(())
}

/// Adds a new named configuration entry.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn add_config(
  config: &mut SetupConfig,
  name: &str,
  entry: ConfigEntry,
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  if has_config(config, name)
    && !confirm(
      &format!("Warning: Configuration '{name}' already exists. Overwrite?"),
      yes,
      input,
      output,
    )
  {
    writeln!(output, "Aborted.").ok();
    return Ok(());
  }

  insert_config(config, name, entry);
  let path = save_config(config)?;

  let e = &config.configs[name];
  writeln!(
    output,
    "Configuration '{name}' added successfully to {}",
    path.display()
  )
  .ok();
  writeln!(output, "Configuration details:").ok();
  write!(output, "{}", format_entry(e)).ok();

  Ok(())
}

/// Removes a named configuration entry.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn remove_config(
  config: &mut SetupConfig,
  name: &str,
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  let Some(e) = config.configs.get(name) else {
    writeln!(output, "\nWarning: Config '{name}' not found.\n").ok();
    return Ok(());
  };

  writeln!(output, "Config {name}").ok();
  writeln!(output, "Configuration details:").ok();
  write!(output, "{}", format_entry(e)).ok();

  if !confirm(
    "\nAre you sure you want to remove this config?",
    yes,
    input,
    output,
  ) {
    writeln!(output, "Aborted.").ok();
    return Ok(());
  }

  remove_config_entry(config, name);
  let path = save_config(config)?;
  writeln!(output, "\nConfig '{name}' was successfully removed").ok();
  writeln!(output, "Configuration saved to: {}\n", path.display()).ok();
  Ok(())
}

/// Lists all saved configuration entries.
pub fn list_configs(config: &SetupConfig, output: &mut impl Write) {
  if config.configs.is_empty() {
    writeln!(output, "  No configurations created.").ok();
    writeln!(
      output,
      "  Run with --init-config to create a default configuration."
    )
    .ok();
    return;
  }

  writeln!(output, "Configurations:").ok();
  for (name, e) in &config.configs {
    writeln!(output, "\n{name}:").ok();
    write!(output, "{}", format_entry(e)).ok();
  }
}
