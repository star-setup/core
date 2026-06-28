use crate::{
  cli::{args::Command, resolve_with_config, Args, ConfigAction, ProfileAction, WorkspaceAction},
  commands::{mono_repo_mode, single_repo_mode},
  config::{
    add_config, create_default_config, list_configs, load_config, remove_config, ConfigEntry,
  },
  ctx::{DryRunRunner, IoCtx, ProcessRunner, RunCtx, Runner},
  interactive::interactive_mode,
  profile::{add_profile, list_profiles, remove_profile},
  utils::check_prerequisites,
  workspace::{clean_workspace, resolve_workspace, status_workspace, update_workspace},
};
use clap::Parser;
use std::{
  error::Error,
  io::{self, IsTerminal},
  path::{Path, PathBuf},
};

const CONFIG_FILE_NAME: &str = ".star-setup.json";

/// Helper to quickly execute a workspace/repo task with the correct runner
fn with_runner<F>(io: IoCtx, f: F) -> Result<(), Box<dyn Error>>
where
  F: FnOnce(&mut RunCtx) -> Result<(), Box<dyn Error>>,
{
  let mut dry = DryRunRunner;
  let mut real = ProcessRunner;
  let runner: &mut dyn Runner = if io.dry_run { &mut dry } else { &mut real };
  let mut ctx = RunCtx { io, runner };
  f(&mut ctx)
}

/// Runs the setup process.
/// # Errors
/// Returns an error if the configuration file is missing or corrupted.
#[allow(clippy::too_many_lines)]
pub fn run() -> Result<(), Box<dyn Error>> {
  let mut stdin = io::stdin().lock();
  let mut stdout = io::stdout();
  let is_terminal = stdin.is_terminal() && stdout.is_terminal();

  let locations = vec![
    PathBuf::from(CONFIG_FILE_NAME),
    dirs::home_dir()
      .map(|h| h.join(CONFIG_FILE_NAME))
      .unwrap_or_default(),
  ];

  let mut config = load_config(&locations, &mut stdout);
  let raw = Args::parse();

  let mut io = IoCtx {
    input: &mut stdin,
    output: &mut stdout,
    verbose: raw.connection.verbose,
    timing: raw.diagnostic.timing,
    dry_run: raw.diagnostic.dry_run,
  };

  if let Some(command) = raw.command {
    match command {
      Command::Config(cmd) => match cmd.action {
        ConfigAction::Init => {
          create_default_config(PathBuf::from(CONFIG_FILE_NAME), raw.yes, &mut io)?;
        }
        ConfigAction::List => list_configs(&config, &mut io),
        ConfigAction::Remove { name } => {
          remove_config(&mut config, &name, raw.yes, &mut io)?;
        }
        ConfigAction::Add {
          name,
          connection,
          build,
          mono,
          diagnostic,
        } => {
          let entry = ConfigEntry::from_flags(&connection, &build, &mono, &diagnostic);
          add_config(&mut config, &name, entry, raw.yes, &mut io)?;
        }
      },
      Command::Profile(cmd) => match cmd.action {
        ProfileAction::List => list_profiles(&config, &mut io),
        ProfileAction::Remove { name } => {
          remove_profile(&mut config, &name, raw.yes, &mut io)?;
        }
        ProfileAction::Add { name, repos } => {
          let vals = std::iter::once(name).chain(repos).collect::<Vec<_>>();
          add_profile(&mut config, &vals, raw.yes, &mut io)?;
        }
      },
      Command::Workspace(ws_cmd) => match ws_cmd.action {
        WorkspaceAction::Update {
          path,
          mono_dir,
          build_dir,
        } => {
          let ws = resolve_workspace(path.as_deref(), mono_dir.as_deref(), build_dir.as_deref())?;
          with_runner(io, |ctx| {
            update_workspace(&ws, ctx).map_err(Box::<dyn Error>::from)
          })?;
        }
        WorkspaceAction::Status {
          path,
          mono_dir,
          build_dir,
          fetch,
        } => {
          let ws = resolve_workspace(path.as_deref(), mono_dir.as_deref(), build_dir.as_deref())?;
          let mut status_io = io;
          status_io.dry_run = false;
          with_runner(status_io, |ctx| {
            status_workspace(&ws, fetch, ctx).map_err(Box::<dyn Error>::from)
          })?;
        }
        WorkspaceAction::Clean {
          path,
          mono_dir,
          build_dir,
        } => {
          let ws = resolve_workspace(path.as_deref(), mono_dir.as_deref(), build_dir.as_deref())?;
          with_runner(io, |ctx| {
            clean_workspace(&ws, ctx).map_err(Box::<dyn Error>::from)
          })?;
        }
      },
    }
    return Ok(());
  }

  let mut args = resolve_with_config(raw, &config).map_err(Box::<dyn Error>::from)?;
  if args.repo.is_none() {
    if is_terminal {
      interactive_mode(&mut args, &mut io)?;
    } else {
      return Err("no repository specified".into());
    }
  }

  check_prerequisites(&mut io)?;

  with_runner(io, |ctx| {
    if args.mono.mono_repo {
      mono_repo_mode(&args, &config, Path::new("."), ctx)?;
    } else {
      single_repo_mode(&args, Path::new("."), ctx)?;
    }
    Ok(())
  })?;

  Ok(())
}
