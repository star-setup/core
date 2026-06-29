use star_setup::workspace::Workspace;
use std::path::{Path, PathBuf};

pub fn make_workspace(root: &Path, repo_dirs: Vec<PathBuf>) -> Workspace {
  Workspace {
    root: root.to_path_buf(),
    repos_path: root.join("repos"),
    build_path: root.join("build"),
    repo_dirs,
  }
}
