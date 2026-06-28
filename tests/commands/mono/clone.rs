use super::super::common::{empty_input, make_io, sink, MockRunner};
use star_setup::commands::mono::clone::clone_mono_repos;
use star_setup::ctx::{RunCtx, Runner};

fn run_mono_test<R, F>(mut runner: R, test_logic: F) -> R
where
  R: Runner,
  F: FnOnce(&std::path::Path, &mut RunCtx<'_, '_>),
{
  let tmp = tempfile::TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();

  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  test_logic(tmp.path(), &mut ctx);
  runner
}

#[test]
fn test_clone_mono_repos_calls_clone_for_each_repo() {
  let runner = run_mono_test(MockRunner::new(), |tmp_path, ctx| {
    let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];
    clone_mono_repos(&repos, tmp_path, false, ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 2);
  assert!(runner
    .calls
    .iter()
    .all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "clone"));
}

#[test]
fn test_clone_mono_repos_empty() {
  let runner = run_mono_test(MockRunner::new(), |tmp_path, ctx| {
    clone_mono_repos(&[], tmp_path, false, ctx).unwrap();
  });

  assert!(runner.calls.is_empty());
}
