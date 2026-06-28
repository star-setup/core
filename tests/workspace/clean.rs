use crate::common::{empty_input, make_io, sink, MockRunner};
use star_setup::{ctx::RunCtx, workspace::types::Workspace};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_workspace_clean_no_build_dir() {
  let tmp = TempDir::new().unwrap();
  let ws = Workspace {
    root: tmp.path().to_path_buf(),
    repos_path: tmp.path().join("repos"),
    build_path: tmp.path().join("build"),
    repo_dirs: vec![],
  };
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };
  ws.clean(&mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("does not exist"));
}

#[test]
fn test_workspace_clean_removes_build_dir() {
  let tmp = TempDir::new().unwrap();
  let build = tmp.path().join("build");
  fs::create_dir_all(&build).unwrap();
  fs::write(build.join("dummy.txt"), "").unwrap();
  let ws = Workspace {
    root: tmp.path().to_path_buf(),
    repos_path: tmp.path().join("repos"),
    build_path: build.clone(),
    repo_dirs: vec![],
  };
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };
  ws.clean(&mut ctx).unwrap();
  assert!(!build.exists());
}

#[test]
fn test_workspace_clean_dry_run() {
  let tmp = TempDir::new().unwrap();
  let build = tmp.path().join("build");
  fs::create_dir_all(&build).unwrap();
  let ws = Workspace {
    root: tmp.path().to_path_buf(),
    repos_path: tmp.path().join("repos"),
    build_path: build.clone(),
    repo_dirs: vec![],
  };
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: star_setup::ctx::IoCtx {
      input: &mut input,
      output: &mut output,
      verbose: false,
      timing: false,
      dry_run: true,
    },
    runner: &mut runner,
  };
  ws.clean(&mut ctx).unwrap();
  assert!(build.exists());
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Would remove directory:"));
}
