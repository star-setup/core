#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]

mod cli;
mod commands;
mod config;
mod interactive;
mod profiles;
mod repository;
mod utils;

use config::{load_config, create_default_config, list_configs, add_config, remove_config, ConfigEntry};
use profiles::{list_profiles, add_profile, remove_profile};
use utils::check_prerequisites;
use commands::{single_repo_mode, mono_repo_mode};
use interactive::interactive_mode;
use cli::{Args};

fn main() {
  let mut config = load_config();
  let mut args = match Args::parse_with_config(&config) {
    Ok(args) => args,
    Err(e) => { eprintln!("Error: {e}"); std::process::exit(1); }
  };

  if args.config.init_config {
    if let Err(e) = create_default_config() {
      eprintln!("Error: {e}"); std::process::exit(1);
    }
    return;
  }
  if args.config.list_configs   { list_configs(&config); return; }
  if args.profile.list_profiles { list_profiles(&config); return; }

  if let Some(name) = args.config.config_remove.as_deref() {
    if let Err(e) = remove_config(&mut config, name) {
      eprintln!("Error: {e}"); std::process::exit(1);
    }
    return;
  }
  if let Some(name) = args.config.config_add.as_deref() {
    let entry = ConfigEntry {
      ssh:         args.connection.ssh,
      build_type:  args.build.build_type.clone(),
      build_dir:   args.build.build_dir.clone(),
      mono_dir:    args.mono.mono_dir.clone(),
      no_build:    args.build.no_build,
      clean:       args.build.clean,
      verbose:     args.connection.verbose,
      cmake_flags: args.cmake_flags.clone(),
    };
    if let Err(e) = add_config(&mut config, name, entry) {
      eprintln!("Error: {e}"); std::process::exit(1);
    }
    return;
  }
  if let Some(name) = args.profile.profile_remove.as_deref()  {
    if let Err(e) = remove_profile(&mut config, name) {
      eprintln!("Error: {e}"); std::process::exit(1);
    }
    return;
  }
  if let Some(vals) = args.profile.profile_add.as_ref() {
    if let Err(e) = add_profile(&mut config, vals) {
      eprintln!("Error: {e}"); std::process::exit(1);
    }
    return;
  }

  if args.repo.is_none() {
    interactive_mode(&mut args);
  }

  if let Err(e) = check_prerequisites(args.connection.verbose) {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }

  let result = if args.mono.mono_repo {
    mono_repo_mode(&args, &config)
  } else {
    single_repo_mode(&args)
  };

  if let Err(e) = result {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }
}
