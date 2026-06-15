//! Configuration file management for ecosystem-setup.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Represents a single named configuration entry.
#[derive(Serialize, Deserialize, Default)]
pub struct ConfigEntry {
  pub ssh:        bool,
  pub build_type: String,
  pub build_dir:  String,
  pub mono_dir:   String,
  pub no_build:   bool,
  pub verbose:    bool,
  pub cmake_args: Vec<String>,
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
  let locations = vec![
    PathBuf::from(".ecosystem-setup.json"),
    dirs::home_dir().unwrap_or_default().join(".ecosystem-setup.json"),
  ];

  let mut invalid_count = 0;

  for path in locations {
    if !path.exists() { continue; }
    match fs::read_to_string(&path) {
      Ok(contents) => match serde_json::from_str::<EcosystemConfig>(&contents) {
        Ok(mut config) => { config.path = Some(path); return config; }
        Err(e) => { println!("Warning: Invalid JSON in {}: {}", path.display(), e); invalid_count += 1; }
      },
      Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
        println!("Error: No permission to read {}", path.display()); invalid_count += 1;
      }
      Err(e) => { println!("An unexpected error occurred reading {}: {}", path.display(), e); invalid_count += 1; }
    }
  }

  if invalid_count != 0 {
    println!(
      "Found {} config file{} that had errors", invalid_count, if invalid_count != 1 { "s" } else { "" }
    );
  } else {
    println!("Failed to find config file");
  }
  EcosystemConfig::new()
}

pub fn save_config(config: &mut EcosystemConfig) -> Result<PathBuf, String> {
  if config.path.is_none() {
    config.path = Some(PathBuf::from(".ecosystem-setup.json"));
  }
  let path = config.path.as_ref().unwrap().clone();
  let json = serde_json::to_string_pretty(config)
    .map_err(|e| format!("Failed to serialize config: {}", e))?;
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
    if input.trim().to_lowercase() != "y" {
      println!("Aborted.");
      return Ok(());
    }
  }

  let mut config = EcosystemConfig::new();
  config.path = Some(path.clone());
  config.configs.insert("default".to_string(), ConfigEntry {
    ssh:        false,
    build_type: "Debug".to_string(),
    build_dir:  "build".to_string(),
    mono_dir:   "build-mono".to_string(),
    no_build:   false,
    verbose:    false,
    cmake_args: vec![],
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
    print!("Warning: Configuration '{}' already exists. Overwrite? (y/n): ", name);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    if input.trim().to_lowercase() != "y" {
      println!("Aborted.");
      return Ok(());
    }
  }

  config.configs.insert(name.to_string(), entry);
  let path = save_config(config)?;

  let e = &config.configs[name];
  println!("Configuration '{}' added successfully to {}", name, path.display());
  println!("Configuration details:");
  println!("  SSH: {}", e.ssh);
  println!("  Build Type: {}", e.build_type);
  println!("  Build Directory: {}", e.build_dir);
  println!("  Mono-build Directory: {}", e.mono_dir);
  println!("  No-build flag: {}", e.no_build);
  println!("  Verbose flag: {}", e.verbose);
  if !e.cmake_args.is_empty() {
    if e.cmake_args.len() == 1 {
      println!("  CMake argument: {}", e.cmake_args[0]);
    } else {
      println!("  CMake arguments:");
      for arg in &e.cmake_args { println!("    {}", arg); }
    }
  }

  Ok(())
}

/// Removes a named configuration entry.
pub fn remove_config(config: &mut EcosystemConfig, name: &str) -> Result<(), String> {
  let e = match config.configs.get(name) {
    None => { println!("\nWarning: Config '{}' not found.\n", name); return Ok(()); }
    Some(e) => e,
  };

  println!("Config {}", name);
  println!("Configuration details:");
  println!("  SSH: {}", e.ssh);
  println!("  Build Type: {}", e.build_type);
  println!("  Build Directory: {}", e.build_dir);
  println!("  Mono-build Directory: {}", e.mono_dir);
  println!("  No-build flag: {}", e.no_build);
  println!("  Verbose flag: {}", e.verbose);

  print!("\nAre you sure you want to remove this config? (y/n): ");
  io::stdout().flush().ok();
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  if input.trim().to_lowercase() != "y" {
    println!("Aborted.");
    return Ok(());
  }

  config.configs.remove(name);
  let path = save_config(config)?;
  println!("\nConfig '{}' was successfully removed", name);
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
    println!("\n{}:", name);
    println!("  SSH: {}", e.ssh);
    println!("  Build Type: {}", e.build_type);
    println!("  Build Directory: {}", e.build_dir);
    println!("  Mono-build Directory: {}", e.mono_dir);
    println!("  No-build flag: {}", e.no_build);
    println!("  Verbose flag: {}", e.verbose);
    if e.cmake_args.is_empty() {
      println!();
    } else if e.cmake_args.len() == 1 {
      println!("  CMake argument: {}", e.cmake_args[0]);
    } else {
      println!("  CMake arguments:");
      for arg in &e.cmake_args { println!("    {}", arg); }
    }
  }
}
