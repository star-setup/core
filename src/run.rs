use crate::{
  cli::{args::Command, resolve_with_config, Args},
  commands::{
    handle_config_cmd, handle_profile_cmd, handle_workspace_cmd, mono_repo_mode, single_repo_mode,
  },
  config::load_config,
  ctx::{with_runner, IoCtx, RunFlags},
  interactive::interactive_mode,
  utils::check_prerequisites,
};
use clap::Parser;
use std::{
  error::Error,
  io::{self, IsTerminal},
  path::{Path, PathBuf},
};

/// Runs the setup process.
/// # Errors
/// Returns an error if the configuration file is missing or corrupted.
pub fn run(config_path: PathBuf) -> Result<(), Box<dyn Error>> {
  let mut stdin = io::stdin().lock();
  let mut stdout = io::stdout();
  let is_terminal = stdin.is_terminal() && stdout.is_terminal();

  let locations = [
    Some(config_path.as_path()),
    dirs::home_dir()
      .as_deref()
      .map(|h| h.join(&config_path))
      .as_deref(),
  ]
  .into_iter()
  .flatten()
  .map(Path::to_path_buf)
  .collect::<Vec<_>>();

  let mut config = load_config(&locations, &mut stdout);
  let raw = Args::parse();

  let mut io = IoCtx {
    input: &mut stdin,
    output: &mut stdout,
  };

  let mut flags = RunFlags {
    verbose: raw.connection.verbose,
    timing: raw.diagnostic.timing,
    dry_run: raw.diagnostic.dry_run,
  };

  if let Some(cmd) = raw.command {
    match cmd {
      Command::Config(c) => {
        handle_config_cmd(c.action, &mut config, config_path, raw.yes, &mut io, &flags)?;
      }
      Command::Profile(p) => {
        handle_profile_cmd(p.action, &mut config, raw.yes, &mut io, &flags)?;
      }
      Command::Workspace(w) => handle_workspace_cmd(w.action, io, flags)?,
    }
    return Ok(());
  }

  let mut args = resolve_with_config(raw, &config).map_err(Box::<dyn Error>::from)?;
  if args.repo.is_none() {
    if is_terminal {
      interactive_mode(&mut args, &mut io, &mut flags)?;
    } else {
      return Err("no repository specified".into());
    }
  }

  check_prerequisites(&mut io, &flags)?;

  with_runner(io, flags, |ctx| {
    if args.mono.mono_repo {
      mono_repo_mode(&args, &config, Path::new("."), ctx)?;
    } else {
      single_repo_mode(&args, Path::new("."), ctx)?;
    }
    Ok(())
  })
}
