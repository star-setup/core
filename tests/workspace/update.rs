use crate::common::{empty_input, make_io, sink, MockRunner};
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
fn test_update_workspace_empty() {
  let ws = make_workspace(vec![]);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx { io: make_io(&mut input, &mut output), runner: &mut runner };
  star_setup::workspace::update_workspace(&ws, &mut ctx).unwrap();
  assert!(runner.calls.is_empty());
}

#[test]
fn test_update_workspace_pulls_each_repo() {
  let ws = make_workspace(vec![
    PathBuf::from("build-mono/repos/user-lib1"),
    PathBuf::from("build-mono/repos/user-lib2"),
  ]);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx { io: make_io(&mut input, &mut output), runner: &mut runner };
  star_setup::workspace::update_workspace(&ws, &mut ctx).unwrap();
  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls.iter().all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}
