use star_setup::commands::create_mono_repo_cmakelists;
#[path = "../../common/mod.rs"]
mod common;
use common::sink;

// create_mono_repo_cmakelists tests
#[test]
fn test_create_mono_repo_cmakelists_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();

  let repos = vec![
    "user-testrepo".to_string(),
    "user/lib1".to_string(),
    "user/lib2".to_string(),
  ];
  create_mono_repo_cmakelists(tmp.path(), &repos, &mut sink()).unwrap();

  let cmake_file = tmp.path().join("CMakeLists.txt");
  assert!(cmake_file.exists());

  let content = std::fs::read_to_string(&cmake_file).unwrap();
  assert!(content.contains("user-testrepo"));
  assert!(content.contains("user-lib1"));
  assert!(content.contains("user-lib2"));
}

#[test]
fn test_create_mono_repo_cmakelists_empty_repos() {
  let tmp = tempfile::TempDir::new().unwrap();
  let repos = vec!["user-testrepo".to_string()];
  create_mono_repo_cmakelists(tmp.path(), &repos, &mut sink()).unwrap();
  assert!(tmp.path().join("CMakeLists.txt").exists());
}
