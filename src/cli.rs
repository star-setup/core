//! Command-line argument parsing for ecosystem-setup.

use clap::{Args as ClapArgs, Parser};
use crate::config::EcosystemConfig;

/// Connection and output flags.
#[derive(ClapArgs)]
pub struct ConnectionFlags {
  /// Use SSH instead of HTTPS for cloning
  #[arg(long)]
  pub ssh: bool,

  /// Show detailed command output
  #[arg(short = 'v', long)]
  pub verbose: bool,
}

/// `CMake` build flags.
#[derive(ClapArgs)]
pub struct BuildFlags {
  /// `CMake` build type
  #[arg(short = 'b', long)]
  pub build_type: Option<String>,

  /// Build directory name
  #[arg(short = 'd', long)]
  pub build_dir: Option<String>,

  /// Skip building, only configure
  #[arg(short = 'n', long)]
  pub no_build: bool,

  /// Clean build directory before building
  #[arg(short = 'c', long)]
  pub clean: bool,
}

/// Mono-repo flags.
#[derive(ClapArgs)]
pub struct MonoRepoFlags {
  /// Mono-repo mode
  #[arg(long)]
  pub mono_repo: bool,

  /// Directory name for mono-repo cloning
  #[arg(long)]
  pub mono_dir: Option<String>,

  /// List of library repositories to clone in mono-repo mode
  #[arg(long, num_args = 1..)]
  pub repos: Option<Vec<String>>,

  /// Use saved profile for library repositories
  #[arg(long)]
  pub profile: Option<String>,
}

/// Config management flags.
#[derive(ClapArgs)]
pub struct ConfigFlags {
  /// Create a default config file in the current directory
  #[arg(long)]
  pub init_config: bool,

  /// Add a new config
  #[arg(long)]
  pub config_add: Option<String>,

  /// Remove a saved configuration
  #[arg(long)]
  pub config_remove: Option<String>,

  /// List all saved configs
  #[arg(long)]
  pub list_configs: bool,
}

/// Profile management flags.
#[derive(ClapArgs)]
pub struct ProfileFlags {
  /// Add a new profile: NAME REPO1 [REPO2 ...]
  #[arg(long, num_args = 2..)]
  pub profile_add: Option<Vec<String>>,

  /// Remove a saved profile
  #[arg(long)]
  pub profile_remove: Option<String>,

  /// List all saved profiles
  #[arg(long)]
  pub list_profiles: bool,
}


#[derive(Parser)]
#[command(
  name = "ecos",
  about = "Lightweight CLI to clone, configure, and wire single or multi-repo ecosystems",
  long_about = None,
)]
pub struct Args {
  /// Repository name (username/repo) or full GitHub URL
  pub repo: Option<String>,

  /// Additional `CMake` arguments
  #[arg(long = "cmake-arg", action = clap::ArgAction::Append)]
  pub cmake_flags: Vec<String>,

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

impl Args {
  pub fn parse_with_config(config: &EcosystemConfig) -> Self {
    let default = config.configs.get("default");
    let mut args = Args::parse();

    if !args.connection.ssh {
      args.connection.ssh = default.is_some_and(|e| e.ssh);
    }
    if !args.connection.verbose {
      args.connection.verbose = default.is_some_and(|e| e.verbose);
    }
    if !args.build.no_build {
      args.build.no_build = default.is_some_and(|e| e.no_build);
    }

    args.build.build_type = args.build.build_type
      .or_else(|| default.map(|e| e.build_type.clone()))
      .or_else(|| Some("Debug".to_string()));

    args.build.build_dir = args.build.build_dir
      .or_else(|| default.map(|e| e.build_dir.clone()))
      .or_else(|| Some("build".to_string()));

    args.mono.mono_dir = args.mono.mono_dir
      .or_else(|| default.map(|e| e.mono_dir.clone()))
      .or_else(|| Some("build-mono".to_string()));

    if args.cmake_flags.is_empty() {
      args.cmake_flags = default.map_or_else(
        Vec::new, |e| e.cmake_flags.clone()
      );
    }

    if args.mono.repos.is_some() || args.mono.profile.is_some() {
      args.mono.mono_repo = true;
    }

    args
  }
}
