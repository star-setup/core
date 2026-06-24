use crate::cli::build::BuildType;
use crate::config::display::format_entry;
use crate::config::io::save_config;
use crate::config::types::{ConfigEntry, SetupConfig};
use crate::utils::confirm::confirm;
use std::io::{BufRead, Write};
use std::path::PathBuf;

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
    )?
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
      build_type: BuildType::Debug,
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
    )?
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
  )? {
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
