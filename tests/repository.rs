use star_setup::repository::{clone_repository, repo_dir_name, resolve_repo_url};

/// repo_dir_name
#[test]
fn test_repo_dir_name() {
  let cases = vec![
    ("owner/repo"),
    ("owner/repo.git"),
    ("git@github.com:owner/repo.git"),
    ("https://github.com/owner/repo"),
    ("https://github.com/owner/repo.git"),
    ("owner/repo/")
  ];

  for input in cases {
    assert_eq!(repo_dir_name(input), "owner-repo", "Failed for input: {input}");
  }
}

#[test]
fn test_repo_dir_name_no_owner() {
  assert_eq!(repo_dir_name("repo"), "repo");
}

/// resolve_repo_url
#[test]
fn test_resolve_repo_url() {
  let cases = vec![
    ("owner/repo",                        false, "https://github.com/owner/repo.git"),
    ("owner/repo",                        true,  "git@github.com:owner/repo.git"    ),
    ("https://github.com/owner/repo.git", false, "https://github.com/owner/repo.git"),
    ("git@github.com:owner/repo.git",     true,  "git@github.com:owner/repo.git"    ),
    ("owner/repo.git",                    false, "https://github.com/owner/repo.git"),
  ];

  for (input, use_ssh, expected) in cases {
    assert_eq!(
      resolve_repo_url(input, use_ssh),
      expected,
      "Failed for input: {input} (use_ssh: {use_ssh})"
    );
  }
}

/// clone_repository
#[test]
fn test_clone_skips_existing_directory() {
  let tmp = std::env::temp_dir().join("star_setup_test_clone");
  std::fs::create_dir_all(&tmp).ok();

  let repo_dir = tmp.join("owner-repo");
  std::fs::create_dir_all(&repo_dir).unwrap();

  let result = clone_repository("owner/repo", &tmp, false, false);
  assert!(result.is_ok());
  assert!(repo_dir.exists());

  std::fs::remove_dir_all(&tmp).ok();
}
