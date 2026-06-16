use star_setup::repository::{clone_repository, repo_dir_name, resolve_repo_url};

/// repo_dir_name
#[test]
fn test_repo_dir_name_shorthand() {
  assert_eq!(repo_dir_name("owner/repo"), "owner-repo");
}

#[test]
fn test_repo_dir_name_shorthand_git_suffix() {
  assert_eq!(repo_dir_name("owner/repo.git"), "owner-repo");
}

#[test]
fn test_repo_dir_name_ssh() {
  assert_eq!(repo_dir_name("git@github.com:owner/repo.git"), "owner-repo");
}

#[test]
fn test_repo_dir_name_https() {
  assert_eq!(repo_dir_name("https://github.com/owner/repo"), "owner-repo");
}

#[test]
fn test_repo_dir_name_https_git_suffix() {
  assert_eq!(
    repo_dir_name("https://github.com/owner/repo.git"),
    "owner-repo"
  );
}

#[test]
fn test_repo_dir_name_trailing_slash() {
  assert_eq!(repo_dir_name("owner/repo/"), "owner-repo");
}

#[test]
fn test_repo_dir_name_no_owner() {
    assert_eq!(repo_dir_name("repo"), "repo");
}

/// resolve_repo_url
#[test]
fn test_resolve_repo_url_shorthand_https() {
  assert_eq!(
    resolve_repo_url("owner/repo", false),
    "https://github.com/owner/repo.git"
  );
}

#[test]
fn test_resolve_repo_url_shorthand_ssh() {
  assert_eq!(
    resolve_repo_url("owner/repo", true),
    "git@github.com:owner/repo.git"
  );
}

#[test]
fn test_resolve_repo_url_full_https_passthrough() {
  let url = "https://github.com/owner/repo.git";
  assert_eq!(resolve_repo_url(url, false), url);
}

#[test]
fn test_resolve_repo_url_full_ssh_passthrough() {
  let url = "git@github.com:owner/repo.git";
  assert_eq!(resolve_repo_url(url, true), url);
}

#[test]
fn test_resolve_repo_url_strips_git_suffix() {
  assert_eq!(
    resolve_repo_url("owner/repo.git", false),
    "https://github.com/owner/repo.git"
  );
}

/// clone_repository
#[test]
fn test_clone_skips_existing_directory() {
  let tmp = std::env::temp_dir().join("star_setup_test_clone");
  std::fs::create_dir_all(&tmp).ok();
  let repo_dir = tmp.join("owner-repo");
  std::fs::create_dir_all(&repo_dir).unwrap();

  // Should return Ok without calling git since dir exists
  // We verify by checking the dir still exists and wasn't modified
  let result = clone_repository("owner/repo", &tmp, false, false);
  assert!(result.is_ok());
  assert!(repo_dir.exists());

  std::fs::remove_dir_all(&tmp).ok();
}
