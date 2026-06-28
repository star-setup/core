use crate::{
  cli::{ConfigAction, ProfileAction, WorkspaceAction},
  config::{
    add_config, create_default_config, list_configs, remove_config, ConfigEntry, SetupConfig,
  },
  ctx::{with_runner, IoCtx},
  profile::{add_profile, list_profiles, remove_profile},
  workspace::{clean_workspace, resolve_workspace, status_workspace, update_workspace},
};
use std::{error::Error, path::PathBuf};

/// Handles configuration-related subcommands.
/// # Errors
/// Returns an error if configuration initializing, addition, or removal fails.
pub fn handle_config_cmd(
  action: ConfigAction,
  config: &mut SetupConfig,
  config_path: PathBuf,
  yes: bool,
  io: &mut IoCtx,
) -> Result<(), Box<dyn Error>> {
  match action {
    ConfigAction::Init => create_default_config(config_path, yes, io)?,
    ConfigAction::List => list_configs(config, io),
    ConfigAction::Remove { name } => remove_config(config, &name, yes, io)?,
    ConfigAction::Add {
      name,
      connection,
      build,
      mono,
      diagnostic,
    } => {
      let entry = ConfigEntry::from_flags(&connection, &build, &mono, &diagnostic);
      add_config(config, &name, entry, yes, io)?;
    }
  }
  Ok(())
}

/// Handles profile-related subcommands.
/// # Errors
/// Returns an error if adding or removing profiles encounters an I/O or validation failure.
pub fn handle_profile_cmd(
  action: ProfileAction,
  config: &mut SetupConfig,
  yes: bool,
  io: &mut IoCtx,
) -> Result<(), Box<dyn Error>> {
  match action {
    ProfileAction::List => list_profiles(config, io),
    ProfileAction::Remove { name } => remove_profile(config, &name, yes, io)?,
    ProfileAction::Add { name, repos } => {
      let vals = std::iter::once(name).chain(repos).collect::<Vec<_>>();
      add_profile(config, &vals, yes, io)?;
    }
  }
  Ok(())
}

/// Handles workspace-related subcommands.
/// # Errors
/// Returns an error if resolving, updating, cleaning, or fetching status for the workspace fails.
pub fn handle_workspace_cmd(action: WorkspaceAction, io: IoCtx) -> Result<(), Box<dyn Error>> {
  match action {
    WorkspaceAction::Update {
      path,
      mono_dir,
      build_dir,
    } => {
      let ws = resolve_workspace(path.as_deref(), mono_dir.as_deref(), build_dir.as_deref())?;
      with_runner(io, |ctx| update_workspace(&ws, ctx).map_err(Into::into))?;
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
        status_workspace(&ws, fetch, ctx).map_err(Into::into)
      })?;
    }
    WorkspaceAction::Clean {
      path,
      mono_dir,
      build_dir,
    } => {
      let ws = resolve_workspace(path.as_deref(), mono_dir.as_deref(), build_dir.as_deref())?;
      with_runner(io, |ctx| clean_workspace(&ws, ctx).map_err(Into::into))?;
    }
  }
  Ok(())
}
