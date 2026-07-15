use crate::{
  cli::{
    ConfigAction, ProfileAction,
    WorkspaceAction::{self, Clean, Status, Update},
  },
  config::{add_config, create_default_config, list_configs, remove_config, Config, ConfigEntry},
  ctx::{with_runner, IoCtx, RunFlags},
  profile::{add_profile, list_profiles, remove_profile, Profile},
  workspace::resolve_workspace,
};
use std::{collections::HashMap, error::Error, path::PathBuf};

/// Handles configuration-related subcommands.
/// # Errors
/// Returns an error if configuration initializing, addition, or removal fails.
pub fn handle_config_cmd(
  action: ConfigAction,
  config: &mut Config,
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
  config: &mut Config,
  yes: bool,
  io: &mut IoCtx,
  flags: RunFlags,
) -> Result<(), Box<dyn Error>> {
  match action {
    ProfileAction::List => list_profiles(config, io),
    ProfileAction::Remove { name } => remove_profile(config, &name, yes, io, flags)?,
    ProfileAction::Add { name, test_repo, repos } => {
      let test_repos = test_repo
        .map(|r| HashMap::from([("default".to_string(), r)]))
        .unwrap_or_default();
      let deps = if repos.is_empty() {
        HashMap::new()
      } else {
        HashMap::from([("default".to_string(), repos)])
      };
      let profile = Profile::from_args(test_repos, deps)?;
      add_profile(config, &name, &profile, yes, io, flags)?;
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
    Update { target } | Clean { target } | Status { target, .. } => target,
  };
  let ws = resolve_workspace(
    target.path.as_deref(),
    target.mono_dir.as_deref(),
    target.build_dir.as_deref(),
    &mut io,
    flags.verbose,
  )?;

  match action {
    Update { .. } => with_runner(io, flags, |ctx| ws.update(ctx).map_err(Into::into)),
    Status { fetch, .. } => {
      with_runner(io, flags, |ctx| ws.status(*fetch, ctx).map_err(Into::into))
    }
    Clean { .. } => with_runner(io, flags, |ctx| ws.clean(ctx).map_err(Into::into)),
  }
}
