use std::path::PathBuf;

/// Resolved display paths for the setup completion summary.
pub struct SetupPaths {
  /// Canonicalized path to the mono-repo root directory.
  pub mono_repo_disp: PathBuf,
  /// Canonicalized paths to the test repositories executables, if found.
  pub exe_paths: Vec<(String, PathBuf)>,
  /// Canonicalized path to the build output directory, if no canonical map was provided.
  pub build_disp: Option<PathBuf>,
}
