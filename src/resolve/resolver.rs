use crate::{
  build::BuildType,
  cli::{Args, BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags},
  config::{Config, ConfigEntry},
  ctx::RunFlags,
  resolve::{ResolvedArgs, ResolvedBuildFlags, ResolvedConnectionFlags, ResolvedMonoFlags},
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

/// Resolves a positive/negative flag pair, each using the other as its negation.
fn resolve_flag_pair(
  pos: bool,
  neg: bool,
  cfg_pos: Option<bool>,
  cfg_neg: Option<bool>,
) -> (bool, bool) {
  (
    resolve_bool(pos, neg, cfg_pos, false),
    resolve_bool(neg, pos, cfg_neg, false),
  )
}

fn resolve_connection_flags(
  c: &ConnectionFlags,
  default: Option<&ConfigEntry>,
) -> ResolvedConnectionFlags {
  ResolvedConnectionFlags {
    ssh: resolve_bool(c.ssh, c.https, default.map(|e| e.ssh), false),
  }
}

fn resolve_run_flags(d: &DiagnosticFlags, default: Option<&ConfigEntry>) -> RunFlags {
  RunFlags {
    verbose: resolve_bool(d.verbose, d.no_verbose, default.map(|e| e.verbose), false),
    timing: resolve_bool(d.timing, d.no_timing, default.map(|e| e.timing), false),
    dry_run: resolve_bool(d.dry_run, d.no_dry_run, default.map(|e| e.dry_run), false),
  }
}

fn resolve_build_flags(
  b: BuildFlags,
  default: Option<&ConfigEntry>,
) -> Result<ResolvedBuildFlags, String> {
  let (watch, no_watch) = resolve_flag_pair(
    b.watch,
    b.no_watch,
    default.map(|e| e.watch),
    default.map(|e| e.no_watch),
  );
  let (dev, no_dev) = resolve_flag_pair(
    b.dev,
    b.no_dev,
    default.map(|e| e.dev),
    default.map(|e| e.no_dev),
  );
  let cmake_flags = Some(b.cmake_flags)
    .filter(|f| !f.is_empty())
    .unwrap_or_else(|| default.map_or_else(Vec::new, |e| e.cmake_flags.clone()));
  let meson_flags = Some(b.meson_flags)
    .filter(|f| !f.is_empty())
    .unwrap_or_else(|| default.map_or_else(Vec::new, |e| e.meson_flags.clone()));

  Ok(ResolvedBuildFlags {
    build_type: match b.build_type {
      Some(s) => s.parse::<BuildType>()?,
      None => default.map(|e| e.build_type).unwrap_or_default(),
    },
    build_dir: b
      .build_dir
      .or_else(|| default.map(|e| e.build_dir.clone()))
      .unwrap_or_else(|| "build".to_string()),
    build_system: b.build_system,
    no_build: resolve_bool(b.no_build, b.build, default.map(|e| e.no_build), false),
    clean: resolve_bool(b.clean, b.no_clean, default.map(|e| e.clean), false),
    watch,
    no_watch,
    dev,
    no_dev,
    cmake_flags,
    meson_flags,
  })
}

fn resolve_mono_flags(mono: MonoRepoFlags, default: Option<&ConfigEntry>) -> ResolvedMonoFlags {
  let deps = mono.deps;
  let profile = mono.profile;
  let mono_repo = mono.mono_repo || deps.is_some() || profile.is_some();
  ResolvedMonoFlags {
    mono_repo,
    mono_dir: mono
      .mono_dir
      .or_else(|| default.map(|e| e.mono_dir.clone()))
      .unwrap_or_else(|| "build-mono".to_string()),
    deps,
    profile,
  }
}

/// Fills the repo from the named profile's `test_repo` when no positional repo
/// was given, so all downstream code sees one resolved repo value.
/// # Errors
/// Returns an error if `profile` names a profile that does not exist.
fn resolve_repo(
  repo: Option<String>,
  profile: Option<&str>,
  config: &Config,
) -> Result<Option<String>, String> {
  if let Some(name) = profile {
    if !config.profiles.contains_key(name) {
      let mut names: Vec<&str> = config.profiles.keys().map(String::as_str).collect();
      names.sort_unstable();
      return Err(format!(
        "Profile '{name}' not found. Available: {}",
        names.join(", ")
      ));
    }
  }
  Ok(repo)
}

/// Resolves raw `Args` into `ResolvedArgs` by applying config defaults and CLI overrides.
/// # Errors
/// Returns an error if the named config does not exist in the provided `SetupConfig`.
pub fn resolve_with_config(args: Args, config: &Config) -> Result<ResolvedArgs, String> {
  let config_name = args.config_name.as_deref().unwrap_or("default");
  let default = config.configs.get(config_name);

  if args.config_name.is_some() && default.is_none() {
    return Err(format!("Configuration '{config_name}' not found"));
  }

  Ok(ResolvedArgs {
    repo: resolve_repo(args.repo, args.mono.profile.as_deref(), config)?,
    yes: args.yes,
    connection: resolve_connection_flags(&args.connection, default),
    diagnostic: resolve_run_flags(&args.diagnostic, default),
    build: resolve_build_flags(args.build, default)?,
    mono: resolve_mono_flags(args.mono, default),
  })
}
