use std::path::PathBuf;

use crate::cli::{BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags};
use clap::{Parser, Subcommand};

/// Config subcommand.
#[derive(Parser)]
pub struct ConfigCommand {
  #[command(subcommand)]
  pub action: ConfigAction,
}

/// Config subcommand actions.
#[derive(Subcommand)]
pub enum ConfigAction {
  /// Create a default config file in the current directory.
  Init,
  /// Add or overwrite a named configuration entry.
  Add {
    /// Name of the configuration entry.
    name: String,
    #[command(flatten)]
    connection: ConnectionFlags,
    #[command(flatten)]
    build: BuildFlags,
    #[command(flatten)]
    mono: MonoRepoFlags,
    #[command(flatten)]
    diagnostic: DiagnosticFlags,
  },
  /// Remove a named configuration entry.
  Remove {
    /// Name of the configuration entry to remove.
    name: String,
  },
  /// List all saved configuration entries.
  List,
}

/// Profile subcommand.
#[derive(Parser)]
pub struct ProfileCommand {
  #[command(subcommand)]
  pub action: ProfileAction,
}

/// Profile subcommand actions.
#[derive(Subcommand)]
pub enum ProfileAction {
  /// Add or overwrite a named profile.
  Add {
    /// Name of the profile.
    name: String,
    /// Repository list (username/repo ...).
    repos: Vec<String>,
  },
  /// Remove a named profile.
  Remove {
    /// Name of the profile to remove.
    name: String,
  },
  /// List all saved profiles.
  List,
}

/// Workspace subcommand.
#[derive(Parser)]
pub struct WorkspaceCommand {
  #[command(subcommand)]
  pub action: WorkspaceAction,
}

/// Workspace subcommand actions.
#[derive(Subcommand)]
pub enum WorkspaceAction {
  /// Pull latest changes for all repos in the workspace.
  Update {
    /// Workspace root directory (default: current directory).
    #[arg(long)]
    path: Option<PathBuf>,
    /// Mono-repo workspace directory name (default: build-mono).
    #[arg(long)]
    mono_dir: Option<String>,
    #[arg(long)]
    build_dir: Option<String>,
  },
  /// Show status of all repos in the workspace.
  Status {
    #[arg(long)]
    path: Option<PathBuf>,
    #[arg(long)]
    mono_dir: Option<String>,
    #[arg(long)]
    build_dir: Option<String>,
    #[arg(long)]
    fetch: bool,
  },
  /// Remove the build directory from the workspace.
  Clean {
    #[arg(long)]
    path: Option<PathBuf>,
    #[arg(long)]
    mono_dir: Option<String>,
    #[arg(long)]
    build_dir: Option<String>,
  },
}
