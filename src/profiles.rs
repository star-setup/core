//! Profile management for ecosystem-setup.

use std::io;
use std::io::Write;
use crate::config::{EcosystemConfig, save_config};

/// Adds a new profile to the configuration.
/// args: [name, repo1, repo2, ...]
pub fn add_profile(config: &mut EcosystemConfig, args: &[String]) -> Result<(), String> {
  if args.len() < 2 {
    return Err("--profile-add requires NAME REPO1 [REPO2 ...]".to_string());
  }

  let name = args[0].clone();
  let repos = args[1..].to_vec();

  if config.profiles.contains_key(&name) {
    print!("Warning: Profile '{}' already exists. Overwrite? (y/n): ", name);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    if input.trim().to_lowercase() != "y" {
      println!("Aborted.");
      return Ok(());
    }
  }

  config.profiles.insert(name.clone(), repos);
  let path = save_config(config)?;
  let repos = config.profiles.get(&name).unwrap();

  println!("Profile '{}' added successfully", name);
  println!("Configuration saved to: {}", path.display());
  println!("Profile details:");
  println!("  Repositories ({}):", repos.len());
  for repo in repos { println!("    - {}", repo); }
  println!("\nUsage: ecos username/test-repo --profile {}", name);

  Ok(())
}

/// Removes a profile from the configuration.
pub fn remove_profile(config: &mut EcosystemConfig, name: &str) -> Result<(), String> {
  let repos = match config.profiles.get(name) {
    None => { println!("Warning: Profile '{}' not found.", name); return Ok(()); }
    Some(r) => r.clone(),
  };

  println!("Profile '{}'", name);
  println!("  Libraries: {}", repos.len());
  for repo in &repos { println!("    - {}", repo); }

  print!("\nAre you sure you want to remove this profile? (y/n): ");
  io::stdout().flush().ok();
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  if input.trim().to_lowercase() != "y" {
    println!("Aborted.");
    return Ok(());
  }

  config.profiles.remove(name);
  let path = save_config(config)?;
  println!("\nProfile '{}' removed successfully", name);
  println!("Configuration saved to: {}\n", path.display());
  Ok(())
}

/// Lists all configured profiles.
pub fn list_profiles(config: &EcosystemConfig) {
  println!("Available profiles:");

  if config.profiles.is_empty() {
    println!("  No profiles configured.");
    println!("  Run with --init-config to create a default configuration.");
    return;
  }

  println!("Configured profiles:\n");
  for (name, repos) in &config.profiles {
    println!("  {}", name);
    println!("  Repositories ({}):", repos.len());
    for repo in repos { println!("      - {}", repo); }
    println!();
  }
}
