use star_setup::repository::{repo_dir_name, resolve_repo_url};

/* =====     REPO_DIR_NAME     ===== */
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

/* =====     RESOLVE_REPO_URL     ===== */
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
