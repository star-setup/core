use star_setup::commands::{create_mono_repo_cmakelists, resolve_test_repo};

// resolve_test_repo tests
#[test]
fn test_resolve_test_repo_shorthand() {
  assert_eq!(resolve_test_repo("user/repo").unwrap(), "user/repo");
}

#[test]
fn test_resolve_test_repo_trailing_slash() {
  assert_eq!(resolve_test_repo("user/repo/").unwrap(), "user/repo");
}

#[test]
fn test_resolve_test_repo_https_url() {
  assert_eq!(
    resolve_test_repo("https://github.com/user/repo").unwrap(),
    "user/repo"
  );
}

#[test]
fn test_resolve_test_repo_https_url_git_suffix() {
  assert_eq!(
    resolve_test_repo("https://github.com/user/repo.git").unwrap(),
    "user/repo"
  );
}

#[test]
fn test_resolve_test_repo_ssh_url() {
  assert_eq!(
    resolve_test_repo("git@github.com:user/repo.git").unwrap(),
    "user/repo"
  );
}

#[test]
fn test_resolve_test_repo_ssh_url_no_git_suffix() {
  assert_eq!(
    resolve_test_repo("git@github.com:user/repo").unwrap(),
    "user/repo"
  );
}

#[test]
fn test_resolve_test_repo_no_owner_errors() {
  assert!(resolve_test_repo("repo").is_err());
}

#[test]
fn test_resolve_test_repo_non_github_url_errors() {
  assert!(resolve_test_repo("https://gitlab.com/user/repo").is_err());
}

// create_mono_repo_cmakelists tests
#[test]
fn test_create_mono_repo_cmakelists_creates_file() {
  let tmp = std::env::temp_dir().join("star_setup_test_cmakelists");
  std::fs::create_dir_all(&tmp).ok();

  let repos = vec!["user/lib1".to_string(), "user/lib2".to_string()];
  create_mono_repo_cmakelists(&tmp, "user-testrepo", &repos).unwrap();

  let cmake_file = tmp.join("CMakeLists.txt");
  assert!(cmake_file.exists());

  let content = std::fs::read_to_string(&cmake_file).unwrap();
  assert!(content.contains("user-testrepo"));
  assert!(content.contains("user-lib1"));
  assert!(content.contains("user-lib2"));

  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn test_create_mono_repo_cmakelists_empty_repos() {
  let tmp = std::env::temp_dir().join("star_setup_test_cmakelists_empty");
  std::fs::create_dir_all(&tmp).ok();

  create_mono_repo_cmakelists(&tmp, "user-testrepo", &[]).unwrap();
  assert!(tmp.join("CMakeLists.txt").exists());

  std::fs::remove_dir_all(&tmp).ok();
}
