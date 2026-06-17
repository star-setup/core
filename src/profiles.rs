//! Profile management for star-setup.

use crate::config::{save_config, SetupConfig};
use crate::utils::confirm;

pub fn insert_profile(config: &mut SetupConfig, name: &str, repos: Vec<String>) {
  config.profiles.insert(name.to_string(), repos);
}

pub fn remove_profile_entry(config: &mut SetupConfig, name: &str) -> bool {
  config.profiles.remove(name).is_some()
}

pub fn has_profile(config: &SetupConfig, name: &str) -> bool {
  config.profiles.contains_key(name)
}

/// Adds a new profile to the configuration.
/// args: [name, repo1, repo2, ...]
pub fn add_profile(config: &mut SetupConfig, args: &[String], yes: bool) -> Result<(), String> {
  if args.len() < 2 {
    return Err("--profile-add requires NAME REPO1 [REPO2 ...]".to_string());
  }

  let name = args[0].clone();
  let repos = args[1..].to_vec();

  if has_profile(config, &name)
    && !confirm(
      &format!("Warning: Profile '{name}' already exists. Overwrite?"),
      yes,
    )
  {
    println!("Aborted.");
    return Ok(());
  }

  insert_profile(config, &name, repos.clone());
  let path = save_config(config)?;

  println!("Profile '{name}' added successfully");
  println!("Configuration saved to: {}", path.display());
  println!("Profile details:");
  println!("  Repositories ({}):", repos.len());
  for repo in repos {
    println!("    - {repo}");
  }
  println!("\nUsage: star-setup username/test-repo --profile {name}");

  Ok(())
}

/// Removes a profile from the configuration.
pub fn remove_profile(config: &mut SetupConfig, name: &str, yes: bool) -> Result<(), String> {
  let repos = match config.profiles.get(name) {
    None => {
      println!("Warning: Profile '{name}' not found.");
      return Ok(());
    }
    Some(r) => r.clone(),
  };

  println!("Profile '{name}'");
  println!("  Libraries: {}", repos.len());
  for repo in &repos {
    println!("    - {repo}");
  }

  if !confirm(
    &format!("Are you sure you want to remove profile '{name}'?"),
    yes,
  ) {
    println!("Aborted.");
    return Ok(());
  }

  remove_profile_entry(config, name);
  let path = save_config(config)?;
  println!("\nProfile '{name}' removed successfully");
  println!("Configuration saved to: {}\n", path.display());
  Ok(())
}

/// Lists all configured profiles.
pub fn list_profiles(config: &SetupConfig) {
  println!("Available profiles:");

  if config.profiles.is_empty() {
    println!("  No profiles configured.");
    println!("  Run with --init-config to create a default configuration.");
    return;
  }

  println!("Configured profiles:\n");
  for (name, repos) in &config.profiles {
    println!("  {name}");
    println!("  Repositories ({}):", repos.len());
    for repo in repos {
      println!("      - {repo}");
    }
    println!();
  }
}
