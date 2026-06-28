use crate::workspace::Workspace;
use std::{fs, path::Path};

/// Resolves a workspace from optional path overrides.
/// # Errors
/// Returns an error if the workspace root or repos directory does not exist.
pub fn resolve_workspace(
  path: Option<&Path>,
  mono_dir: Option<&str>,
  build_dir: Option<&str>,
) -> Result<Workspace, String> {
  let base = path.unwrap_or_else(|| Path::new("."));
  let root = base.join(mono_dir.unwrap_or("build-mono"));
  let repos_path = root.join("repos");
  let build_path = root.join(build_dir.unwrap_or("build"));

  if !root.exists() {
    return Err(format!("Workspace not found: {}", root.display()));
  }
  if !repos_path.exists() {
    return Err(format!(
      "Repos directory not found: {}",
      repos_path.display()
    ));
  }

  let repo_dirs = fs::read_dir(&repos_path)
    .map_err(|e| format!("Failed to read repos directory: {e}"))?
    .filter_map(Result::ok)
    .map(|entry| entry.path())
    .filter(|p| p.is_dir() && p.join(".git").exists())
    .collect();

  Ok(Workspace {
    root,
    repos_path,
    build_path,
    repo_dirs,
  })
}
