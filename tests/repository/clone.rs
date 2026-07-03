use std::fs::create_dir_all;

use star_setup::{
  ctx::ProcessRunner,
  repository::{clone_repo, clone_repos},
};
use tempfile::TempDir;

use crate::common::{with_ctx, with_ctx_runner, MockRunner};

#[test]
fn test_clone_skips_existing_directory() {
  with_ctx_runner(ProcessRunner, |tmp_path, ctx| {
    let repo_dir = tmp_path.join("owner-repo");
    create_dir_all(repo_dir.join(".git")).unwrap();

    let result = clone_repo("owner/repo", tmp_path, false, true, false, ctx);
    assert!(result.is_ok());
    assert!(repo_dir.exists());
  });
}

#[test]
fn test_clone_repo_calls_git_clone() {
  let tmp = TempDir::new().unwrap();

  let runner = with_ctx_runner(MockRunner::new(), |_, ctx| {
    clone_repo("user/repo", tmp.path(), false, true, false, ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 1);
  let (cmd, cwd) = &runner.calls[0];
  assert_eq!(cmd[0], "git");
  assert_eq!(cmd[1], "clone");
  assert!(cmd[2].contains("user/repo"));
  assert_eq!(cwd.as_deref(), Some(tmp.path()));
}

#[test]
fn test_clone_repos_calls_clone_for_each_repo() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];
    clone_repos(&repos, tmp_path, false, ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 2);
  assert!(runner
    .calls
    .iter()
    .all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "clone"));
}

#[test]
fn test_clone_repos_empty() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    clone_repos(&[], tmp_path, false, ctx).unwrap();
  });

  assert!(runner.calls.is_empty());
}

#[test]
fn test_clone_repos_per_repo_lines_require_verbose() {
  let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    clone_repos(&repos, tmp_path, false, ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Cloning repositories"));
  assert!(!out.contains("Cloning user-repo1"));
  assert!(!out.contains("Finished cloning (2 repositories)"));

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    ctx.flags.verbose = true;
    clone_repos(&repos, tmp_path, false, ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Cloning user-repo1"));
  assert!(out.contains("Finished cloning (2 repositories)"));
}
