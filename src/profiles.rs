//! Profile management.

use crate::config::io::save_config;
use crate::config::types::SetupConfig;
use crate::utils::confirm;
use std::io::{BufRead, Write};

/// Inserts or overwrites a named profile.
pub fn insert_profile(config: &mut SetupConfig, name: &str, repos: Vec<String>) {
  config.profiles.insert(name.to_string(), repos);
}

/// Removes a named profile. Returns `true` if it existed.
pub fn remove_profile_entry(config: &mut SetupConfig, name: &str) -> bool {
  config.profiles.remove(name).is_some()
}

/// Returns `true` if a profile with the given name exists.
#[must_use]
pub fn has_profile(config: &SetupConfig, name: &str) -> bool {
  config.profiles.contains_key(name)
}

/// Adds a new profile to the configuration.
/// args: [name, repo1, repo2, ...]
/// # Errors
/// Returns an error if fewer than two arguments are provided or if saving fails.
pub fn add_profile(
  config: &mut SetupConfig,
  args: &[String],
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  if args.len() < 2 {
    return Err("--profile-add requires NAME REPO1 [REPO2 ...]".to_string());
  }

  let name = args[0].clone();
  let repos = args[1..].to_vec();

  if has_profile(config, &name)
    && !confirm(
      &format!("Warning: Profile '{name}' already exists. Overwrite?"),
      yes,
      input,
      output,
    )?
  {
    writeln!(output, "Aborted.").ok();
    return Ok(());
  }

  insert_profile(config, &name, repos.clone());
  let path = save_config(config)?;

  writeln!(output, "Profile '{name}' added successfully").ok();
  writeln!(output, "Configuration saved to: {}", path.display()).ok();
  writeln!(output, "Profile details:").ok();
  writeln!(output, "  Repositories ({}):", repos.len()).ok();
  for repo in repos {
    writeln!(output, "    - {repo}").ok();
  }
  writeln!(
    output,
    "\nUsage: star-setup username/test-repo --profile {name}"
  )
  .ok();
  Ok(())
}

/// Removes a profile from the configuration.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn remove_profile(
  config: &mut SetupConfig,
  name: &str,
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  let repos = match config.profiles.get(name) {
    None => {
      writeln!(output, "Warning: Profile '{name}' not found.").ok();
      return Ok(());
    }
    Some(r) => r.clone(),
  };

  writeln!(output, "Profile '{name}'").ok();
  writeln!(output, "  Libraries: {}", repos.len()).ok();
  for repo in &repos {
    writeln!(output, "    - {repo}").ok();
  }

  if !confirm(
    &format!("Are you sure you want to remove profile '{name}'?"),
    yes,
    input,
    output,
  )? {
    writeln!(output, "Aborted.").ok();
    return Ok(());
  }

  remove_profile_entry(config, name);
  let path = save_config(config)?;
  writeln!(output, "\nProfile '{name}' removed successfully").ok();
  writeln!(output, "Configuration saved to: {}\n", path.display()).ok();
  Ok(())
}

/// Lists all configured profiles.
pub fn list_profiles(config: &SetupConfig, output: &mut impl Write) {
  writeln!(output, "Available profiles:").ok();

  if config.profiles.is_empty() {
    writeln!(output, "  No profiles configured.").ok();
    writeln!(
      output,
      "  Run with --init-config to create a default configuration."
    )
    .ok();
    return;
  }

  writeln!(output, "Configured profiles:\n").ok();
  for (name, repos) in &config.profiles {
    writeln!(output, "  {name}").ok();
    writeln!(output, "  Repositories ({}):", repos.len()).ok();
    for repo in repos {
      writeln!(output, "      - {repo}").ok();
    }
    writeln!(output).ok();
  }
}
