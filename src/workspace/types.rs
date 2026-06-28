use std::path::PathBuf;

/// A resolved workspace with paths to key directories and cloned repos.
#[derive(Debug)]
pub struct Workspace {
  /// Workspace root directory (e.g. `./build-mono`).
  pub root: PathBuf,
  /// Path to the cloned repositories directory.
  pub repos_path: PathBuf,
  /// Path to the build output directory.
  pub build_path: PathBuf,
  /// Paths to each cloned repository directory.
  pub repo_dirs: Vec<PathBuf>,
}
