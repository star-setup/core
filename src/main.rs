mod utils;
mod repository;
mod config;
mod profiles;
mod cli;
mod interactive;
mod commands;

use config::{load_config, create_default_config, list_configs, add_config, remove_config, ConfigEntry};
use profiles::{list_profiles, add_profile, remove_profile};
use utils::check_prerequisites;
use commands::{single_repo_mode, mono_repo_mode};
use interactive::interactive_mode;
use cli::Args;

fn main() {
  let mut config = load_config();

  let mut args = Args::parse_with_config(&config);

  if args.init_config    { if let Err(e) = create_default_config()           { eprintln!("Error: {}", e); std::process::exit(1); } return; }
  if args.list_configs   { list_configs(&config); return; }
  if args.list_profiles  { list_profiles(&config); return; }

  if let Some(name) = &args.config_remove.clone() {
    if let Err(e) = remove_config(&mut config, name) { eprintln!("Error: {}", e); std::process::exit(1); }
    return;
  }
  if let Some(name) = &args.config_add.clone() {
    let entry = ConfigEntry {
      ssh:        args.ssh,
      build_type: args.build_type.clone(),
      build_dir:  args.build_dir.clone(),
      mono_dir:   args.mono_dir.clone(),
      no_build:   args.no_build,
      verbose:    args.verbose,
      cmake_args: args.cmake_args.clone(),
    };
    if let Err(e) = add_config(&mut config, name, entry) { eprintln!("Error: {}", e); std::process::exit(1); }
    return;
  }
  if let Some(name) = &args.profile_remove.clone() {
    if let Err(e) = remove_profile(&mut config, name) { eprintln!("Error: {}", e); std::process::exit(1); }
    return;
  }
  if let Some(vals) = &args.profile_add.clone() {
    if let Err(e) = add_profile(&mut config, vals) { eprintln!("Error: {}", e); std::process::exit(1); }
    return;
  }

  if args.repo.is_none() {
    interactive_mode(&mut args);
  }

  if let Err(e) = check_prerequisites(args.verbose) {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }

  let result = if args.mono_repo || args.profile.is_some() {
    mono_repo_mode(&args, &config)
  } else {
    single_repo_mode(&args)
  };

  if let Err(e) = result {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }
}
