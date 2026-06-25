use crate::{
  cli::build::BuildType,
  config::{
    display::format_entry,
    io::save_config,
    types::{ConfigEntry, SetupConfig},
  },
  ctx::IoCtx,
  prompts::confirm,
};
use std::{io::Write, path::PathBuf};

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
pub fn create_default_config(path: PathBuf, yes: bool, io: &mut IoCtx<'_>) -> Result<(), String> {
  if path.exists()
    && !confirm(
      &format!("{} already exists. Overwrite?", path.display()),
      yes,
      io,
    )?
  {
    writeln!(io.output, "Aborted.").ok();
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
      timing: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
  );

  save_config(&mut config)?;

  writeln!(
    io.output,
    "Created config file: {}",
    dunce::canonicalize(&path).unwrap_or(path).display()
  )
  .ok();
  writeln!(io.output, "Edit this file to customize your defaults.").ok();
  writeln!(io.output, "\nConfig files are checked in this order:").ok();
  writeln!(io.output, "  1. ./.star-setup.json (current directory)").ok();
  writeln!(io.output, "  2. ~/.star-setup.json (home directory)").ok();

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
  io: &mut IoCtx<'_>,
) -> Result<(), String> {
  if has_config(config, name)
    && !confirm(
      &format!("Warning: Configuration '{name}' already exists. Overwrite?"),
      yes,
      io,
    )?
  {
    writeln!(io.output, "Aborted.").ok();
    return Ok(());
  }

  insert_config(config, name, entry);
  let path = save_config(config)?;

  let e = &config.configs[name];
  writeln!(
    io.output,
    "Configuration '{name}' added successfully to {}",
    path.display()
  )
  .ok();
  writeln!(io.output, "Configuration details:").ok();
  write!(io.output, "{}", format_entry(e)).ok();

  Ok(())
}

/// Removes a named configuration entry.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn remove_config(
  config: &mut SetupConfig,
  name: &str,
  yes: bool,
  io: &mut IoCtx<'_>,
) -> Result<(), String> {
  let Some(e) = config.configs.get(name) else {
    writeln!(io.output, "\nWarning: Config '{name}' not found.\n").ok();
    return Ok(());
  };

  writeln!(io.output, "Config {name}").ok();
  writeln!(io.output, "Configuration details:").ok();
  write!(io.output, "{}", format_entry(e)).ok();

  if !confirm("\nAre you sure you want to remove this config?", yes, io)? {
    writeln!(io.output, "Aborted.").ok();
    return Ok(());
  }

  remove_config_entry(config, name);
  let path = save_config(config)?;
  writeln!(io.output, "\nConfig '{name}' was successfully removed").ok();
  writeln!(io.output, "Configuration saved to: {}\n", path.display()).ok();
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
