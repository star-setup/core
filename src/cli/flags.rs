use crate::cli::BuildSystem;
use clap::Args as ClapArgs;

#[allow(clippy::struct_excessive_bools)]
#[derive(ClapArgs)]
pub struct ConnectionFlags {
  /// Use SSH instead of HTTPS for cloning
  #[arg(long, conflicts_with = "https")]
  pub ssh: bool,
  /// Use HTTPS instead of SSH for cloning
  #[arg(long, conflicts_with = "ssh")]
  pub https: bool,

  /// Show detailed command output
  #[arg(short = 'v', long, conflicts_with = "no_verbose")]
  pub verbose: bool,
  /// Suppress detailed command output
  #[arg(long, conflicts_with = "verbose")]
  pub no_verbose: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(ClapArgs)]
pub struct BuildFlags {
  /// Build type
  #[arg(short = 'b', long)]
  pub build_type: Option<String>,

  /// Build directory name
  #[arg(short = 'd', long)]
  pub build_dir: Option<String>,

  /// Build system to use, skipping auto-detection
  #[arg(long, value_name = "BUILD_SYSTEM")]
  pub build_system: Option<BuildSystem>,

  /// Skip building, only configure
  #[arg(short = 'n', long, conflicts_with = "build")]
  pub no_build: bool,
  /// Build after configuring (overrides config `no_build`)
  #[arg(long, conflicts_with = "no_build")]
  pub build: bool,

  /// Clean build directory before building
  #[arg(short = 'c', long, conflicts_with = "no_clean")]
  pub clean: bool,
  /// Do not clean build directory
  #[arg(long, conflicts_with = "clean")]
  pub no_clean: bool,

  /// Additional `CMake` arguments
  #[arg(long = "cmake-arg", action = clap::ArgAction::Append)]
  pub cmake_flags: Vec<String>,

  /// Additional Meson arguments
  #[arg(long = "meson-arg", action = clap::ArgAction::Append)]
  pub meson_flags: Vec<String>,

  /// Automatically open watch scripts for npm mono-repo mode.
  #[arg(long)]
  pub watch: bool,
  /// Skip generating watch scripts for npm mono-repo mode.
  #[arg(long)]
  pub no_watch: bool,
}

#[derive(ClapArgs)]
pub struct MonoRepoFlags {
  /// Mono-repo mode
  #[arg(long)]
  pub mono_repo: bool,

  /// Directory name for mono-repo cloning
  #[arg(long)]
  pub mono_dir: Option<String>,

  /// List of library repositories to clone in mono-repo mode
  #[arg(long, num_args = 1.., conflicts_with = "profile")]
  pub repos: Option<Vec<String>>,

  /// Use saved profile for library repositories
  #[arg(long, conflicts_with = "repos")]
  pub profile: Option<String>,
}

#[derive(ClapArgs)]
pub struct DiagnosticFlags {
  /// Show timing information for each phase
  #[arg(long)]
  pub timing: bool,

  /// If set, print commands instead of executing them without making any changes.
  #[arg(long)]
  pub dry_run: bool,
}
