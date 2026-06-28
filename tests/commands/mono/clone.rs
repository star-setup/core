use super::super::common::{with_runner_ctx, MockRunner};
use star_setup::commands::mono::clone::clone_mono_repos;

#[test]
fn test_clone_mono_repos_calls_clone_for_each_repo() {
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
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
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    clone_mono_repos(&[], tmp_path, false, ctx).unwrap();
  });

  assert!(runner.calls.is_empty());
}
