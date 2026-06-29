use crate::{
  config::{save_config, SetupConfig},
  ctx::{IoCtx, RunFlags},
  profile::print_profile_details,
  prompts::confirm_abort,
};

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
  io: &mut IoCtx<'_>,
  flags: &RunFlags,
) -> Result<(), String> {
  if args.len() < 2 {
    return Err("--profile-add requires NAME REPO1 [REPO2 ...]".to_string());
  }

  let name = args[0].clone();
  let repos = args[1..].to_vec();

  if has_profile(config, &name)
    && !confirm_abort(
      &format!("Warning: Profile '{name}' already exists. Overwrite?"),
      yes,
      io,
    )?
  {
    return Ok(());
  }

  if flags.dry_run {
    writeln!(io.output, "Would save profile '{name}' to config file").ok();
  } else {
    insert_profile(config, &name, repos.clone());
    let path = save_config(config)?;
    writeln!(io.output, "Profile '{name}' added successfully").ok();
    writeln!(io.output, "Configuration saved to: {}", path.display()).ok();
  }
  print_profile_details(io.output, "Profile details:", "Repositories", &repos);
  writeln!(
    io.output,
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
  io: &mut IoCtx<'_>,
  flags: &RunFlags,
) -> Result<(), String> {
  let repos = match config.profiles.get(name) {
    None => {
      writeln!(io.output, "Warning: Profile '{name}' not found.").ok();
      return Ok(());
    }
    Some(r) => r.clone(),
  };

  print_profile_details(
    io.output,
    &format!("Profile '{name}'"),
    "Repositories",
    &repos,
  );

  if !confirm_abort(
    &format!("Are you sure you want to remove profile '{name}'?"),
    yes,
    io,
  )? {
    return Ok(());
  }

  if flags.dry_run {
    writeln!(io.output, "Would remove profile '{name}' from config file").ok();
  } else {
    remove_profile_entry(config, name);
    let path = save_config(config)?;
    writeln!(io.output, "\nProfile '{name}' removed successfully").ok();
    writeln!(io.output, "Configuration saved to: {}\n", path.display()).ok();
  }
  Ok(())
}
