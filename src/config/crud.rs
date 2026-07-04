use crate::{
  config::{format_entry, persist_or_dry_run, save_config, ConfigEntry, SetupConfig},
  ctx::{IoCtx, RunFlags},
  prompts::confirm_abort,
};
use dunce::canonicalize;
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
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  let prompt = if flags.dry_run {
    format!(
      "  {} already exists. Overwrite? [DRY-RUN]: No changes will be made",
      path.display()
    )
  } else {
    format!("  {} already exists. Overwrite?", path.display())
  };
  if path.exists() && !confirm_abort(&prompt, yes, io)? {
    return Ok(());
  }

  if flags.dry_run {
    writeln!(
      io.output,
      "  Would create config file: {}",
      canonicalize(&path).unwrap_or(path).display()
    )
    .ok();
  } else {
    let mut config = SetupConfig::new();
    config.path = Some(path.clone());
    config
      .configs
      .insert("default".to_string(), ConfigEntry::default());

    let path = save_config(&mut config, flags.timing, &mut io.output)?;

    writeln!(
      io.output,
      "  Created config file: {}",
      canonicalize(&path).unwrap_or(path).display()
    )
    .ok();
  }

  writeln!(io.output, "  Edit this file to customize your defaults.\n").ok();
  writeln!(io.output, "  Config files are checked in this order:").ok();
  writeln!(io.output, "    1. ./.star-setup.json (current directory)").ok();
  writeln!(io.output, "    2. ~/.star-setup.json (home directory)").ok();

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
  flags: RunFlags,
) -> Result<(), String> {
  if has_config(config, name)
    && !confirm_abort(
      &format!("  Warning: Configuration '{name}' already exists. Overwrite?"),
      yes,
      io,
    )?
  {
    return Ok(());
  }

  persist_or_dry_run(
    config,
    flags,
    io,
    &format!("Would save configuration '{name}' to config file"),
    |config| insert_config(config, name, entry),
    |config, path, io| {
      writeln!(
        io.output,
        "  Configuration '{name}' added successfully to {}",
        path.display()
      )
      .ok();
      let e: &ConfigEntry = &config.configs[name];
      writeln!(io.output, "  Configuration details:").ok();
      write!(io.output, "  {}", format_entry(e)).ok();
    },
  )
}

/// Removes a named configuration entry.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn remove_config(
  config: &mut SetupConfig,
  name: &str,
  yes: bool,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  let Some(e) = config.configs.get(name) else {
    writeln!(io.output, "  Warning: Config '{name}' not found.\n").ok();
    return Ok(());
  };

  writeln!(io.output, "  Config {name}").ok();
  writeln!(io.output, "  Configuration details:").ok();
  write!(io.output, "  {}", format_entry(e)).ok();

  if !confirm_abort("  Are you sure you want to remove this config?", yes, io)? {
    return Ok(());
  }

  persist_or_dry_run(
    config,
    flags,
    io,
    &format!("Would remove configuration '{name}' from config file"),
    |config| {
      remove_config_entry(config, name);
    },
    |_config, path, io| {
      writeln!(io.output, "  Config '{name}' was successfully removed").ok();
      writeln!(io.output, "  Configuration saved to: {}", path.display()).ok();
    },
  )
}
