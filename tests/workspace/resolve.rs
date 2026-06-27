use star_setup::workspace::resolve_workspace;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_resolve_workspace_errors_when_missing() {
  let tmp = TempDir::new().unwrap();
  let result = resolve_workspace(Some(tmp.path()), None, None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Workspace not found"));
}

#[test]
fn test_resolve_workspace_errors_when_no_repos() {
  let tmp = TempDir::new().unwrap();
  fs::create_dir_all(tmp.path().join("build-mono")).unwrap();
  let result = resolve_workspace(Some(tmp.path()), None, None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Repos directory not found"));
}

#[test]
fn test_resolve_workspace_succeeds() {
  let tmp = TempDir::new().unwrap();
  fs::create_dir_all(tmp.path().join("build-mono").join("repos")).unwrap();
  let result = resolve_workspace(Some(tmp.path()), None, None);
  assert!(result.is_ok());
  let ws = result.unwrap();
  assert_eq!(ws.repo_dirs.len(), 0);
}

#[test]
fn test_resolve_workspace_finds_repos() {
  let tmp = TempDir::new().unwrap();
  let repos = tmp.path().join("build-mono").join("repos");
  fs::create_dir_all(repos.join("user-lib1")).unwrap();
  fs::create_dir_all(repos.join("user-lib2")).unwrap();
  let ws = resolve_workspace(Some(tmp.path()), None, None).unwrap();
  assert_eq!(ws.repo_dirs.len(), 2);
}

#[test]
fn test_resolve_workspace_custom_mono_dir() {
  let tmp = TempDir::new().unwrap();
  fs::create_dir_all(tmp.path().join("my-workspace").join("repos")).unwrap();
  let result = resolve_workspace(Some(tmp.path()), Some("my-workspace"), None);
  assert!(result.is_ok());
}

#[test]
fn test_resolve_workspace_custom_build_dir() {
  let tmp = TempDir::new().unwrap();
  fs::create_dir_all(tmp.path().join("build-mono").join("repos")).unwrap();
  let ws = resolve_workspace(Some(tmp.path()), None, Some("out")).unwrap();
  assert!(ws.build_path.ends_with("out"));
}
