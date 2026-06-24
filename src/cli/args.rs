use crate::{
  cli::{
    flags::{BuildFlags, ConfigFlags, ConnectionFlags, MonoRepoFlags, ProfileFlags},
    resolve::resolve_with_config,
    resolved::ResolvedArgs,
  },
  config::types::SetupConfig,
};
use clap::Parser;

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
  /// Parses CLI arguments and resolves them against the provided `SetupConfig`.
  /// # Errors
  /// Returns an error if the named config does not exist in the loaded `SetupConfig`.
  pub fn parse_with_config(config: &SetupConfig) -> Result<ResolvedArgs, String> {
    resolve_with_config(Args::parse(), config)
  }
}
