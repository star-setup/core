use crate::{
  cli::{
    Args, BuildType, ResolvedArgs, ResolvedBuildFlags, ResolvedConnectionFlags,
    ResolvedDiagnosticFlags, ResolvedMonoFlags,
  },
  config::SetupConfig,
};

/// Resolves a boolean flag from CLI positive/negative flags, config value, and a default.
/// Negative flag takes highest priority, then positive, then config, then default.
#[must_use]
pub fn resolve_bool(positive: bool, negative: bool, config: Option<bool>, default: bool) -> bool {
  if negative {
    return false;
  }
  if positive {
    return true;
  }
  match config {
    Some(val) => val,
    None => default,
  }
}

/// Resolves raw `Args` into `ResolvedArgs` by applying config defaults and CLI overrides.
/// # Errors
/// Returns an error if the named config does not exist in the provided `SetupConfig`.
pub fn resolve_with_config(mut args: Args, config: &SetupConfig) -> Result<ResolvedArgs, String> {
  let config_name = args.config_name.as_deref().unwrap_or("default");
  let default = config.configs.get(config_name);

  if args.config_name.is_some() && default.is_none() {
    return Err(format!("Configuration '{config_name}' not found"));
  }

  let ssh = resolve_bool(
    args.connection.ssh,
    args.connection.https,
    default.map(|e| e.ssh),
    false,
  );
  let verbose = resolve_bool(
    args.connection.verbose,
    args.connection.no_verbose,
    default.map(|e| e.verbose),
    false,
  );
  let timing = resolve_bool(
    args.diagnostic.timing,
    false,
    default.map(|e| e.timing),
    false,
  );
  let dry_run = resolve_bool(
    args.diagnostic.dry_run,
    false,
    default.map(|e| e.dry_run),
    false,
  );
  let no_build = resolve_bool(
    args.build.no_build,
    args.build.build,
    default.map(|e| e.no_build),
    false,
  );
  let clean = resolve_bool(
    args.build.clean,
    args.build.no_clean,
    default.map(|e| e.clean),
    false,
  );

  let cmake_flags = Some(args.build.cmake_flags)
    .filter(|f| !f.is_empty())
    .unwrap_or_else(|| default.map_or_else(Vec::new, |e| e.cmake_flags.clone()));

  let meson_flags = Some(args.build.meson_flags)
    .filter(|f| !f.is_empty())
    .unwrap_or_else(|| default.map_or_else(Vec::new, |e| e.meson_flags.clone()));

  let repos = args.mono.repos.take();
  let profile = args.mono.profile.take();
  let mono_repo = args.mono.mono_repo || repos.is_some() || profile.is_some();

  Ok(ResolvedArgs {
    repo: args.repo,
    yes: args.yes,
    connection: ResolvedConnectionFlags { ssh, verbose },
    diagnostic: ResolvedDiagnosticFlags { timing, dry_run },
    build: ResolvedBuildFlags {
      build_type: match args.build.build_type {
        Some(s) => s.parse::<BuildType>()?,
        None => default.map(|e| e.build_type).unwrap_or_default(),
      },
      build_dir: args
        .build
        .build_dir
        .or_else(|| default.map(|e| e.build_dir.clone()))
        .unwrap_or_else(|| "build".to_string()),
      build_system: args.build.build_system,
      no_build,
      clean,
      cmake_flags,
      meson_flags,
      watch: args.build.watch,
      no_watch: args.build.no_watch,
    },
    mono: ResolvedMonoFlags {
      mono_repo,
      mono_dir: args
        .mono
        .mono_dir
        .or_else(|| default.map(|e| e.mono_dir.clone()))
        .unwrap_or_else(|| "build-mono".to_string()),
      repos,
      profile,
    },
  })
}
