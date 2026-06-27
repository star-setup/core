use crate::{
  cli::{
    commands::{ConfigCommand, ProfileCommand},
    resolve_with_config, BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags, ResolvedArgs,
  },
  config::SetupConfig,
};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
  /// Manage saved configurations.
  Config(ConfigCommand),
  /// Manage saved profiles.
  Profile(ProfileCommand),
}

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

  /// Skip confirmation prompts (non-interactive mode)
  #[arg(short = 'y', long)]
  pub yes: bool,

  /// Select a named configuration to use
  #[arg(long = "config")]
  pub config_name: Option<String>,

  #[command(subcommand)]
  pub command: Option<Command>,

  #[command(flatten)]
  pub connection: ConnectionFlags,

  #[command(flatten)]
  pub build: BuildFlags,

  #[command(flatten)]
  pub mono: MonoRepoFlags,

  #[command(flatten)]
  pub diagnostic: DiagnosticFlags,
}

impl Args {
  /// Parses CLI arguments and resolves them against the provided `SetupConfig`.
  /// # Errors
  /// Returns an error if the named config does not exist in the loaded `SetupConfig`.
  pub fn parse_with_config(config: &SetupConfig) -> Result<ResolvedArgs, String> {
    resolve_with_config(Args::parse(), config)
  }
}
