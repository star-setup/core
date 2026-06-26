use super::super::common::{empty_input, make_io, sink, MockRunner};
use star_setup::commands::mono::clone::clone_mono_repos;
use star_setup::ctx::RunCtx;
use tempfile::TempDir;

#[test]
fn test_clone_mono_repos_calls_clone_for_each_repo() {
  let tmp = TempDir::new().unwrap();
  let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  clone_mono_repos(&repos, tmp.path(), false, &mut ctx).unwrap();

  assert_eq!(runner.calls.len(), 2);
  assert!(runner
    .calls
    .iter()
    .all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "clone"));
}

#[test]
fn test_clone_mono_repos_empty() {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  clone_mono_repos(&[], tmp.path(), false, &mut ctx).unwrap();

  assert!(runner.calls.is_empty());
}
