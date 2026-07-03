use crate::{
  cli::{ConfigAction, ProfileAction, WorkspaceAction},
  config::{
    add_config, create_default_config, list_configs, remove_config, ConfigEntry, SetupConfig,
  },
  ctx::{with_runner, IoCtx, RunFlags},
  profile::{add_profile, list_profiles, remove_profile},
  workspace::resolve_workspace,
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
  flags: RunFlags,
) -> Result<(), Box<dyn Error>> {
  match action {
    ConfigAction::Init => create_default_config(config_path, yes, io, flags)?,
    ConfigAction::List => list_configs(config, io),
    ConfigAction::Remove { name } => remove_config(config, &name, yes, io, flags)?,
    ConfigAction::Add {
      name,
      connection,
      build,
      mono,
      diagnostic,
    } => {
      let entry = ConfigEntry::from_flags(&connection, &build, &mono, &diagnostic);
      add_config(config, &name, entry, yes, io, flags)?;
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
  flags: RunFlags,
) -> Result<(), Box<dyn Error>> {
  match action {
    ProfileAction::List => list_profiles(config, io),
    ProfileAction::Remove { name } => remove_profile(config, &name, yes, io, flags)?,
    ProfileAction::Add { name, repos } => {
      let vals = std::iter::once(name).chain(repos).collect::<Vec<_>>();
      add_profile(config, &vals, yes, io, flags)?;
    }
  }
  Ok(())
}

/// Handles workspace-related subcommands.
/// # Errors
/// Returns an error if resolving, updating, cleaning, or fetching status for the workspace fails.
pub fn handle_workspace_cmd(
  action: &WorkspaceAction,
  mut io: IoCtx,
  flags: RunFlags,
) -> Result<(), Box<dyn Error>> {
  let target = match &action {
    WorkspaceAction::Update { target }
    | WorkspaceAction::Clean { target }
    | WorkspaceAction::Status { target, .. } => target,
  };
  let ws = resolve_workspace(
    target.path.as_deref(),
    target.mono_dir.as_deref(),
    target.build_dir.as_deref(),
    &mut io,
    flags.verbose,
  )?;

  match action {
    WorkspaceAction::Update { .. } => {
      with_runner(io, flags, |ctx| ws.update(ctx).map_err(Into::into))
    }
    WorkspaceAction::Status { fetch, .. } => {
      with_runner(io, flags, |ctx| ws.status(*fetch, ctx).map_err(Into::into))
    }
    WorkspaceAction::Clean { .. } => {
      with_runner(io, flags, |ctx| ws.clean(ctx).map_err(Into::into))
    }
  }
}
