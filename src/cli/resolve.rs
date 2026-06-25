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
  config.unwrap_or(default)
}

/// Resolves raw `Args` into `ResolvedArgs` by applying config defaults and CLI overrides.
/// # Errors
/// Returns an error if the named config does not exist in the provided `SetupConfig`.
pub fn resolve_with_config(mut args: Args, config: &SetupConfig) -> Result<ResolvedArgs, String> {
  let config_name = args.config.config_name.as_deref().unwrap_or("default");
  if let Some(name) = &args.config.config_name {
    if !config.configs.contains_key(name.as_str()) {
      return Err(format!("Configuration '{name}' not found"));
    }
  }

  let default = config.configs.get(config_name);

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
  if args.build.cmake_flags.is_empty() {
    args.build.cmake_flags = default.map_or_else(Vec::new, |e| e.cmake_flags.clone());
  }
  if args.build.meson_flags.is_empty() {
    args.build.meson_flags = default.map_or_else(Vec::new, |e| e.meson_flags.clone());
  }

  let repos = args.mono.repos.take();
  let profile = args.mono.profile.take();
  let mono_repo = args.mono.mono_repo || repos.is_some() || profile.is_some();

  Ok(ResolvedArgs {
    repo: args.repo,
    yes: args.yes,
    connection: ResolvedConnectionFlags { ssh, verbose },
    diagnostic: ResolvedDiagnosticFlags { timing },
    build: ResolvedBuildFlags {
      build_type: if let Some(s) = args.build.build_type {
        s.parse::<BuildType>()?
      } else {
        default.map(|e| e.build_type.clone()).unwrap_or_default()
      },
      build_dir: args
        .build
        .build_dir
        .or_else(|| default.map(|e| e.build_dir.clone()))
        .unwrap_or_else(|| "build".to_string()),
      no_build,
      clean,
      cmake_flags: args.build.cmake_flags,
      meson_flags: args.build.meson_flags,
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
    config: args.config,
    profile: args.profile,
  })
}
