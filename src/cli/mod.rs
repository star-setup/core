pub mod args;
pub use args::Args;
pub mod build;
pub use build::{detect_build_system, detect_mono_build_system, BuildSystem, BuildType};
pub mod flags;
pub use flags::{BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags};
pub mod resolve;
pub use resolve::{resolve_bool, resolve_with_config};
pub mod resolved;
pub use resolved::{
  ResolvedArgs, ResolvedBuildFlags, ResolvedConnectionFlags, ResolvedDiagnosticFlags,
  ResolvedMonoFlags,
};
pub mod commands;
pub use commands::{
  ConfigAction, ConfigCommand, ProfileAction, ProfileCommand, WorkspaceAction, WorkspaceCommand,
};
