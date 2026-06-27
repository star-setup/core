use crate::common::{empty_input, make_io, MockRunner};
use star_setup::{ctx::RunCtx, workspace::resolve::Workspace};
use std::path::PathBuf;

fn make_workspace(repo_dirs: Vec<PathBuf>) -> Workspace {
  Workspace {
    root: PathBuf::from("build-mono"),
    repos_path: PathBuf::from("build-mono/repos"),
    build_path: PathBuf::from("build-mono/build"),
    repo_dirs,
  }
}

#[test]
fn test_status_workspace_empty() {
  let ws = make_workspace(vec![]);
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };
  star_setup::workspace::status_workspace(&ws, false, &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Workspace status:"));
}

#[test]
fn test_status_workspace_shows_repos() {
  let ws = make_workspace(vec![PathBuf::from("build-mono/repos/user-lib1")]);
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut runner = MockRunner::new();
  runner.capture_responses.push_back("main".to_string());
  runner.capture_responses.push_back(String::new());
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };
  star_setup::workspace::status_workspace(&ws, false, &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("user-lib1"));
  assert!(out.contains("clean"));
}

#[test]
fn test_status_workspace_with_fetch() {
  let ws = make_workspace(vec![PathBuf::from("build-mono/repos/user-lib1")]);
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut runner = MockRunner::new();
  runner.capture_responses.push_back("main".to_string());
  runner.capture_responses.push_back(String::new());
  runner.capture_responses.push_back("2".to_string());
  runner.capture_responses.push_back("1".to_string());
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };
  star_setup::workspace::status_workspace(&ws, true, &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("↑2 ↓1"));
  assert!(runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "fetch"));
}
