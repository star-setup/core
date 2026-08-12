use std::path::PathBuf;

use crate::cli::{BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags};
use clap::{Args as ClapArgs, Parser, Subcommand};

/// Config subcommand.
#[derive(Parser)]
pub struct ConfigCommand {
  #[command(subcommand)]
  pub action: ConfigAction,
}

/// Config subcommand actions.
#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
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
    /// Test repositories (user/game1 user/game2).
    #[arg(long, num_args = 1..)]
    test_repos: Vec<String>,
    /// Dependency repositories (user/lib1 user/lib2).
    #[arg(long, num_args = 1..)]
    deps: Vec<String>,
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

#[derive(ClapArgs)]
pub struct WorkspaceTarget {
  /// Workspace root directory (default: current directory).
  #[arg(long)]
  pub path: Option<PathBuf>,
  /// Mono-repo workspace directory name (default: build-mono).
  #[arg(long)]
  pub mono_dir: Option<String>,
  #[arg(long)]
  pub build_dir: Option<String>,
}

#[derive(Subcommand)]
pub enum WorkspaceAction {
  /// Pull latest changes for all repos in the workspace.
  Update {
    #[command(flatten)]
    target: WorkspaceTarget,
  },
  /// Show status of all repos in the workspace.
  Status {
    #[command(flatten)]
    target: WorkspaceTarget,
    #[arg(long)]
    fetch: bool,
  },
  /// Remove the build directory from the workspace.
  Clean {
    #[command(flatten)]
    target: WorkspaceTarget,
  },
}
