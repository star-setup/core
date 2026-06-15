//! Configuration file management for ecosystem-setup.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Represents a single named configuration entry.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Default)]
pub struct ConfigEntry {
  pub ssh:         bool,
  pub build_type:  String,
  pub build_dir:   String,
  pub mono_dir:    String,
  pub no_build:    bool,
  pub clean:       bool,
  pub verbose:     bool,
  pub cmake_flags: Vec<String>,
}

/// Top-level configuration structure.
#[derive(Serialize, Deserialize, Default)]
pub struct EcosystemConfig {
  #[serde(default)]
  pub configs:  HashMap<String, ConfigEntry>,
  #[serde(default)]
  pub profiles: HashMap<String, Vec<String>>,
  #[serde(skip)]
  pub path: Option<PathBuf>,
}

impl EcosystemConfig {
  pub fn new() -> Self {
    Self::default()
  }
}

pub fn load_config() -> EcosystemConfig {
  let mut locations = vec![PathBuf::from(".ecosystem-setup.json")];
  if let Some(home) = dirs::home_dir() {
      locations.push(home.join(".ecosystem-setup.json"));
  }

  let mut invalid_count = 0;

  for path in locations {
    if !path.exists() { continue; }
    match fs::read_to_string(&path) {
      Ok(contents) => match serde_json::from_str::<EcosystemConfig>(&contents) {
        Ok(mut config) => {
          config.path = Some(path);
          return config;
        }
        Err(e) => {
          println!("Warning: Invalid JSON in {}: {e}", path.display());
          invalid_count += 1;
        }
      },
      Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
        println!("Error: No permission to read {}", path.display());
        invalid_count += 1;
      }
      Err(e) => {
        println!("An unexpected error occurred reading {}: {e}", path.display());
        invalid_count += 1;
      }
    }
  }

  if invalid_count != 0 {
    println!(
      "Found {invalid_count} config file{} that had errors", if invalid_count == 1 { "" } else { "s" }
    );
  }
  EcosystemConfig::new()
}

pub fn save_config(config: &mut EcosystemConfig) -> Result<PathBuf, String> {
  let path = config.path.get_or_insert_with(|| PathBuf::from(".ecosystem-setup.json")).clone();
  let json = serde_json::to_string_pretty(config)
    .map_err(|e| format!("Failed to serialize config: {e}"))?;

  fs::write(&path, json).map_err(|e| match e.kind() {
    io::ErrorKind::PermissionDenied => format!("Error: No permission to write to {}", path.display()),
    _ => format!("An unexpected error occurred writing {}: {}", path.display(), e),
  })?;
  Ok(path)
}

/// Creates a default configuration file in the current directory.
pub fn create_default_config() -> Result<(), String> {
  let path = PathBuf::from(".ecosystem-setup.json");

  if path.exists() {
    print!("{} already exists. Overwrite? (y/n): ", path.display());
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    if !input.trim().eq_ignore_ascii_case("y") {
      println!("Aborted.");
      return Ok(());
    }
  }

  let mut config = EcosystemConfig::new();
  config.path = Some(path.clone());
  config.configs.insert("default".to_string(), ConfigEntry {
    ssh:         false,
    build_type:  "Debug".to_string(),
    build_dir:   "build".to_string(),
    mono_dir:    "build-mono".to_string(),
    no_build:    false,
    clean:       false,
    verbose:     false,
    cmake_flags: vec![],
  });

  save_config(&mut config)?;

  println!("Created config file: {}", path.canonicalize().unwrap_or(path).display());
  println!("Edit this file to customize your defaults.");
  println!("\nConfig files are checked in this order:");
  println!("  1. ./.ecosystem-setup.json (current directory)");
  println!("  2. ~/.ecosystem-setup.json (home directory)");

  Ok(())
}

/// Adds a new named configuration entry.
pub fn add_config(config: &mut EcosystemConfig, name: &str, entry: ConfigEntry) -> Result<(), String> {
  if config.configs.contains_key(name) {
    print!("Warning: Configuration '{name}' already exists. Overwrite? (y/n): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    if !input.trim().eq_ignore_ascii_case("y") {
      println!("Aborted.");
      return Ok(());
    }
  }

  config.configs.insert(name.to_string(), entry);
  let path = save_config(config)?;

  let e = &config.configs[name];
  println!("Configuration '{name}' added successfully to {}", path.display());
  println!("Configuration details:");
  println!("  SSH: {}", e.ssh);
  println!("  Build Type: {}", e.build_type);
  println!("  Build Directory: {}", e.build_dir);
  println!("  Mono-build Directory: {}", e.mono_dir);
  println!("  No-build flag: {}", e.no_build);
  println!("  Clean flag: {}", e.clean);
  println!("  Verbose flag: {}", e.verbose);
  if !e.cmake_flags.is_empty() {
    if e.cmake_flags.len() == 1 {
      println!("  CMake argument: {}", e.cmake_flags[0]);
    } else {
      println!("  CMake arguments:");
      for arg in &e.cmake_flags { println!("    {arg}"); }
    }
  }

  Ok(())
}

/// Removes a named configuration entry.
pub fn remove_config(config: &mut EcosystemConfig, name: &str) -> Result<(), String> {
  let Some(e) = config.configs.get(name) else {
    println!("\nWarning: Config '{name}' not found.\n");
    return Ok(());
  };

  println!("Config {name}");
  println!("Configuration details:");
  println!("  SSH: {}", e.ssh);
  println!("  Build Type: {}", e.build_type);
  println!("  Build Directory: {}", e.build_dir);
  println!("  Mono-build Directory: {}", e.mono_dir);
  println!("  No-build flag: {}", e.no_build);
  println!("  Clean flag: {}", e.clean);
  println!("  Verbose flag: {}", e.verbose);

  print!("\nAre you sure you want to remove this config? (y/n): ");
  io::stdout().flush().ok();
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  if !input.trim().eq_ignore_ascii_case("y") {
    println!("Aborted.");
    return Ok(());
  }

  config.configs.remove(name);
  let path = save_config(config)?;
  println!("\nConfig '{name}' was successfully removed");
  println!("Configuration saved to: {}\n", path.display());
  Ok(())
}

/// Lists all saved configuration entries.
pub fn list_configs(config: &EcosystemConfig) {
  if config.configs.is_empty() {
    println!("  No configurations created.");
    println!("  Run with --init-config to create a default configuration.");
    return;
  }

  println!("Configurations:");
  for (name, e) in &config.configs {
    println!("\n{name}:");
    println!("  SSH: {}", e.ssh);
    println!("  Build Type: {}", e.build_type);
    println!("  Build Directory: {}", e.build_dir);
    println!("  Mono-build Directory: {}", e.mono_dir);
    println!("  No-build flag: {}", e.no_build);
    println!("  Clean flag: {}", e.clean);
    println!("  Verbose flag: {}", e.verbose);
    if e.cmake_flags.is_empty() {
      println!();
    } else if e.cmake_flags.len() == 1 {
      println!("  CMake argument: {}", e.cmake_flags[0]);
    } else {
      println!("  CMake arguments:");
      for arg in &e.cmake_flags { println!("    {arg}"); }
    }
  }
}
