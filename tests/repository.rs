mod common;
use common::{with_runner_ctx, MockRunner};
use star_setup::{
  ctx::ProcessRunner,
  repository::{clone_repository, repo_dir_name, resolve_repo_url},
};

#[test]
fn test_repo_dir_name() {
  let cases = [
    "owner/repo",
    "owner/repo.git",
    "git@github.com:owner/repo.git",
    "https://github.com/owner/repo",
    "https://github.com/owner/repo.git",
    "owner/repo/",
  ];

  for input in cases {
    assert_eq!(
      repo_dir_name(input),
      "owner-repo",
      "Failed for input: {input}"
    );
  }
}

#[test]
fn test_repo_dir_name_no_owner() {
  assert_eq!(repo_dir_name("repo"), "repo");
}

#[test]
fn test_resolve_repo_url() {
  let cases = vec![
    ("owner/repo", false, "https://github.com/owner/repo.git"),
    ("owner/repo/", false, "https://github.com/owner/repo.git"),
    ("owner/repo", true, "git@github.com:owner/repo.git"),
    ("owner/repo/", true, "git@github.com:owner/repo.git"),
    (
      "https://github.com/owner/repo.git",
      false,
      "https://github.com/owner/repo.git",
    ),
    (
      "https://github.com/owner/repo.git",
      true,
      "https://github.com/owner/repo.git",
    ),
    (
      "git@github.com:owner/repo.git",
      true,
      "git@github.com:owner/repo.git",
    ),
    ("owner/repo.git", false, "https://github.com/owner/repo.git"),
  ];

  for (input, use_ssh, expected) in cases {
    assert_eq!(
      resolve_repo_url(input, use_ssh),
      expected,
      "Failed for input: {input} (use_ssh: {use_ssh})"
    );
  }
}

#[test]
fn test_clone_skips_existing_directory() {
  with_runner_ctx(ProcessRunner, |tmp_path, ctx| {
    let repo_dir = tmp_path.join("owner-repo");
    std::fs::create_dir_all(&repo_dir).unwrap();

    let result = clone_repository("owner/repo", tmp_path, false, ctx);
    assert!(result.is_ok());
    assert!(repo_dir.exists());
  });
}

#[test]
fn test_clone_repository_calls_git_clone() {
  let tmp = tempfile::TempDir::new().unwrap();

  let runner = with_runner_ctx(MockRunner::new(), |_, ctx| {
    clone_repository("user/repo", tmp.path(), false, ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 1);
  let (cmd, cwd) = &runner.calls[0];
  assert_eq!(cmd[0], "git");
  assert_eq!(cmd[1], "clone");
  assert!(cmd[2].contains("user/repo"));
  assert_eq!(cwd.as_deref(), Some(tmp.path()));
}
