use clap::Parser;
pub mod build;
pub mod flags;
pub mod resolved;
use crate::config::SetupConfig;
pub use build::{detect_build_system, BuildSystem, BuildType};
pub use flags::{BuildFlags, ConfigFlags, ConnectionFlags, MonoRepoFlags, ProfileFlags};
pub use resolved::{ResolvedArgs, ResolvedBuildFlags, ResolvedConnectionFlags, ResolvedMonoFlags};

/// Top-level CLI arguments for star-setup.
#[derive(Parser)]
#[command(
  name = "star-setup",
  about = "Lightweight CLI to clone, configure, and wire single or multi-repo ecosystems",
  long_about = None,
)]
pub struct Args {
  /// Repository name (username/repo) or full GitHub URL
  pub repo: Option<String>,

  /// Additional `CMake` arguments
  #[arg(long = "cmake-arg", action = clap::ArgAction::Append)]
  pub cmake_flags: Vec<String>,

  /// Skip confirmation prompts (non-interactive mode)
  #[arg(short = 'y', long)]
  pub yes: bool,

  #[command(flatten)]
  pub connection: ConnectionFlags,

  #[command(flatten)]
  pub build: BuildFlags,

  #[command(flatten)]
  pub mono: MonoRepoFlags,

  #[command(flatten)]
  pub config: ConfigFlags,

  #[command(flatten)]
  pub profile: ProfileFlags,
}

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
  if args.cmake_flags.is_empty() {
    args.cmake_flags = default.map_or_else(Vec::new, |e| e.cmake_flags.clone());
  }

  let repos = args.mono.repos.take();
  let profile = args.mono.profile.take();
  let mono_repo = args.mono.mono_repo || repos.is_some() || profile.is_some();

  Ok(ResolvedArgs {
    repo: args.repo,
    cmake_flags: args.cmake_flags,
    yes: args.yes,
    connection: ResolvedConnectionFlags { ssh, verbose },
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

impl Args {
  /// Parses CLI arguments and resolves them against the provided `SetupConfig`.
  /// # Errors
  /// Returns an error if the named config does not exist in the loaded `SetupConfig`.
  pub fn parse_with_config(config: &SetupConfig) -> Result<ResolvedArgs, String> {
    resolve_with_config(Args::parse(), config)
  }
}
