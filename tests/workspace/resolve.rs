use star_setup::workspace::resolve_workspace;
use std::{fs, path::Path};
use tempfile::TempDir;

fn with_tmp_dir(f: impl FnOnce(&Path)) {
  let tmp = TempDir::new().unwrap();
  f(tmp.path());
}

#[test]
fn test_resolve_workspace_errors_when_missing() {
  with_tmp_dir(|path| {
    let result = resolve_workspace(Some(path), None, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Workspace not found"));
  });
}

#[test]
fn test_resolve_workspace_errors_when_no_repos() {
  with_tmp_dir(|path| {
    fs::create_dir_all(path.join("build-mono")).unwrap();
    let result = resolve_workspace(Some(path), None, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Repos directory not found"));
  });
}

#[test]
fn test_resolve_workspace_succeeds() {
  with_tmp_dir(|path| {
    fs::create_dir_all(path.join("build-mono").join("repos")).unwrap();
    let result = resolve_workspace(Some(path), None, None);
    assert!(result.is_ok());
    let ws = result.unwrap();
    assert_eq!(ws.repo_dirs.len(), 0);
  });
}

#[test]
fn test_resolve_workspace_finds_repos() {
  with_tmp_dir(|path| {
    let repos = path.join("build-mono").join("repos");
    fs::create_dir_all(repos.join("user-lib1").join(".git")).unwrap();
    fs::create_dir_all(repos.join("user-lib2").join(".git")).unwrap();
    let ws = resolve_workspace(Some(path), None, None).unwrap();
    assert_eq!(ws.repo_dirs.len(), 2);
  });
}

#[test]
fn test_resolve_workspace_custom_mono_dir() {
  with_tmp_dir(|path| {
    fs::create_dir_all(path.join("my-workspace").join("repos")).unwrap();
    let result = resolve_workspace(Some(path), Some("my-workspace"), None);
    assert!(result.is_ok());
  });
}

#[test]
fn test_resolve_workspace_custom_build_dir() {
  with_tmp_dir(|path| {
    fs::create_dir_all(path.join("build-mono").join("repos")).unwrap();
    let ws = resolve_workspace(Some(path), None, Some("out")).unwrap();
    assert!(ws.build_path.ends_with("out"));
  });
}

#[test]
fn test_resolve_workspace_excludes_non_git_dirs() {
  with_tmp_dir(|path| {
    let repos = path.join("build-mono").join("repos");
    fs::create_dir_all(repos.join("user-lib1").join(".git")).unwrap();
    fs::create_dir_all(repos.join("not-a-repo")).unwrap();
    let ws = resolve_workspace(Some(path), None, None).unwrap();
    assert_eq!(ws.repo_dirs.len(), 1);
  });
}
