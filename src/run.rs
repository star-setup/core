use crate::{
  cli::{Args, ResolvedArgs},
  commands::{mono_repo_mode, single_repo_mode},
  config::{
    add_config, create_default_config, list_configs, load_config, remove_config, ConfigEntry,
    SetupConfig,
  },
  ctx::{IoCtx, ProcessRunner, RunCtx},
  interactive::interactive_mode,
  profile::{add_profile, list_profiles, remove_profile},
  utils::check_prerequisites,
};
use std::{
  error::Error,
  io::{self, IsTerminal},
  path::PathBuf,
};

fn handle_early_commands(
  args: &ResolvedArgs,
  config: &mut SetupConfig,
  io: &mut IoCtx<'_>,
) -> Result<bool, Box<dyn Error>> {
  if args.config.init_config {
    create_default_config(PathBuf::from(".star-setup.json"), args.yes, io)?;
    return Ok(true);
  }

  if args.config.list_configs {
    list_configs(config, io);
    return Ok(true);
  }

  if args.profile.list_profiles {
    list_profiles(config, io.output);
    return Ok(true);
  }

  if let Some(name) = args.config.config_remove.as_deref() {
    remove_config(config, name, args.yes, io)?;
    return Ok(true);
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
      timing: args.diagnostic.timing,
      cmake_flags: args.build.cmake_flags.clone(),
      meson_flags: args.build.meson_flags.clone(),
    };
    add_config(config, name, entry, args.yes, io)?;
    return Ok(true);
  }

  if let Some(name) = args.profile.profile_remove.as_deref() {
    remove_profile(config, name, args.yes, io)?;
    return Ok(true);
  }

  if let Some(vals) = args.profile.profile_add.as_ref() {
    add_profile(config, vals, args.yes, io)?;
    return Ok(true);
  }

  Ok(false)
}

/// Runs the setup process.
/// # Errors
/// Returns an error if the configuration file is missing or corrupted.
pub fn run() -> Result<(), Box<dyn Error>> {
  let mut stdin = io::stdin().lock();
  let mut stdout = io::stdout();

  let mut locations = vec![PathBuf::from(".star-setup.json")];
  if let Some(home) = dirs::home_dir() {
    locations.push(home.join(".star-setup.json"));
  }

  let mut config = load_config(&locations, &mut stdout);
  let mut args = Args::parse_with_config(&config)?;

  {
    let mut early_io = IoCtx {
      input: &mut stdin,
      output: &mut stdout,
      verbose: false,
      timing: false,
    };
    if handle_early_commands(&args, &mut config, &mut early_io)? {
      return Ok(());
    }
    if args.repo.is_none() {
      if io::stdin().is_terminal() {
        interactive_mode(&mut args, &mut early_io)?;
      } else {
        return Err("no repository specified".into());
      }
    }

    check_prerequisites(&mut early_io)?;
  }

  let mut runner = ProcessRunner {
    verbose: args.connection.verbose,
  };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut stdin,
      output: &mut stdout,
      verbose: args.connection.verbose,
      timing: args.diagnostic.timing,
    },
    runner: &mut runner,
  };

  if args.mono.mono_repo {
    mono_repo_mode(&args, &config, &mut ctx)?;
  } else {
    single_repo_mode(&args, &mut ctx)?;
  }

  Ok(())
}
