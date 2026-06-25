use star_setup::{
  ctx::{IoCtx, ProcessRunner, RunCtx},
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
  let tmp = tempfile::TempDir::new().unwrap();
  let repo_dir = tmp.path().join("owner-repo");
  std::fs::create_dir_all(&repo_dir).unwrap();

  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = clone_repository("owner/repo", tmp.path(), false, &mut ctx);
  assert!(result.is_ok());
  assert!(repo_dir.exists());
}
