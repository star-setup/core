//! Command-line argument parsing for ecosystem-setup.

use clap::Parser;
use crate::config::EcosystemConfig;

#[derive(Parser)]
#[command(
  name = "ecos",
  about = "Lightweight CLI to clone, configure, and wire single or multi-repo ecosystems",
  long_about = None,
)]
pub struct Args {
  /// Repository name (username/repo) or full GitHub URL
  pub repo: Option<String>,

  /// Use SSH instead of HTTPS for cloning
  #[arg(long)]
  pub ssh: bool,

  /// Show detailed command output
  #[arg(short = 'v', long)]
  pub verbose: bool,

  /// CMake build type
  #[arg(short = 'b', long, default_value = "Debug")]
  pub build_type: String,

  /// Build directory name
  #[arg(short = 'd', long, default_value = "build")]
  pub build_dir: String,

  /// Skip building, only configure
  #[arg(short = 'n', long)]
  pub no_build: bool,

  /// Clean build directory before building
  #[arg(short = 'c', long)]
  pub clean: bool,

  /// Mono-repo mode
  #[arg(long)]
  pub mono_repo: bool,

  /// Directory name for mono-repo cloning
  #[arg(long, default_value = "build-mono")]
  pub mono_dir: String,

  /// List of library repositories to clone in mono-repo mode
  #[arg(long, num_args = 1..)]
  pub repos: Option<Vec<String>>,

  /// Use saved profile for library repositories
  #[arg(long)]
  pub profile: Option<String>,

  /// Additional CMake arguments
  #[arg(long = "cmake-arg", action = clap::ArgAction::Append)]
  pub cmake_args: Vec<String>,

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

impl Args {
  pub fn parse_with_config(config: &EcosystemConfig) -> Self {
    let default = config.configs.get("default");
    let mut args = Args::parse();

    // Apply config defaults where CLI didn't override
    if !args.ssh      { args.ssh      = default.map_or(false, |e| e.ssh); }
    if !args.verbose  { args.verbose  = default.map_or(false, |e| e.verbose); }
    if !args.no_build { args.no_build = default.map_or(false, |e| e.no_build); }
    if args.build_type == "Debug" { args.build_type = default.map_or_else(|| "Debug".to_string(), |e| e.build_type.clone()); }
    if args.build_dir == "build"  { args.build_dir  = default.map_or_else(|| "build".to_string(),  |e| e.build_dir.clone()); }
    if args.mono_dir == "build-mono" { args.mono_dir = default.map_or_else(|| "build-mono".to_string(), |e| e.mono_dir.clone()); }
    if args.cmake_args.is_empty() { args.cmake_args = default.map_or_else(Vec::new, |e| e.cmake_args.clone()); }

    if args.repos.is_some() || args.profile.is_some() { args.mono_repo = true; }

    args
  }
}
