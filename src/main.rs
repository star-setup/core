//! Entry point. Parses arguments, loads config, and dispatches to the appropriate command handler.

use star_setup::cli::Args;
use star_setup::commands::{mono_repo_mode, single_repo_mode};
use star_setup::config::{
  add_config, create_default_config, list_configs, load_config, remove_config, ConfigEntry,
};
use star_setup::interactive::interactive_mode;
use star_setup::profiles::{add_profile, list_profiles, remove_profile};
use star_setup::utils::check_prerequisites;
use std::io;
use std::io::IsTerminal;
use std::path::PathBuf;

fn main() {
  let mut stdin = io::stdin().lock();
  let mut stdout = io::stdout();

  let mut locations = vec![PathBuf::from(".star-setup.json")];
  if let Some(home) = dirs::home_dir() {
    locations.push(home.join(".star-setup.json"));
  }

  let mut config = load_config(&locations, &mut stdout);
  let mut args = match Args::parse_with_config(&config) {
    Ok(args) => args,
    Err(e) => {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
  };

  if args.config.init_config {
    if let Err(e) = create_default_config(
      PathBuf::from(".star-setup.json"),
      args.yes,
      &mut stdin,
      &mut stdout,
    ) {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
    return;
  }
  if args.config.list_configs {
    list_configs(&config, &mut stdout);
    return;
  }
  if args.profile.list_profiles {
    list_profiles(&config, &mut stdout);
    return;
  }

  if let Some(name) = args.config.config_remove.as_deref() {
    if let Err(e) = remove_config(&mut config, name, args.yes, &mut stdin, &mut stdout) {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
    return;
  }
  if let Some(name) = args.config.config_add.as_deref() {
    let entry = ConfigEntry {
      ssh: args.connection.ssh,
      build_type: args.build.build_type.clone(),
      build_dir: args.build.build_dir.clone(),
      mono_dir: args.mono.mono_dir.clone(),
      no_build: args.build.no_build,
      clean: args.build.clean,
      verbose: args.connection.verbose,
      cmake_flags: args.cmake_flags.clone(),
    };
    if let Err(e) = add_config(&mut config, name, entry, args.yes, &mut stdin, &mut stdout) {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
    return;
  }
  if let Some(name) = args.profile.profile_remove.as_deref() {
    if let Err(e) = remove_profile(&mut config, name, args.yes, &mut stdin, &mut stdout) {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
    return;
  }
  if let Some(vals) = args.profile.profile_add.as_ref() {
    if let Err(e) = add_profile(&mut config, vals, args.yes, &mut stdin, &mut stdout) {
      eprintln!("Error: {e}");
      std::process::exit(1);
    }
    return;
  }

  if args.repo.is_none() {
    if std::io::stdin().is_terminal() {
      if let Err(e) = interactive_mode(&mut args, &mut io::stdin().lock(), &mut io::stdout()) {
        eprintln!("Error: {e}");
        std::process::exit(1);
      }
    } else {
      eprintln!("Error: no repository specified");
      std::process::exit(1);
    }
  }

  if let Err(e) = check_prerequisites(args.connection.verbose, &mut stdout) {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }

  let result = if args.mono.mono_repo {
    mono_repo_mode(&args, &config, &mut stdout)
  } else {
    single_repo_mode(&args, &mut stdin, &mut stdout)
  };

  if let Err(e) = result {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }
}
