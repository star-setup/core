use star_setup::commands::{create_mono_repo_cmakelists, resolve_test_repo};
mod helpers;
use helpers::sink;

// resolve_test_repo tests
#[test]
fn test_reoslve_test_repo() {
  let cases = [
    "user/repo",
    "user/repo/",
    "https://github.com/user/repo",
    "https://github.com/user/repo.git",
    "git@github.com:user/repo.git",
    "git@github.com:user/repo"
  ];

  for input in cases {
    assert_eq!(
      resolve_test_repo(input),
      Ok("user/repo".to_string()),
      "Failed for input: {input}"
    )
  }
}

#[test]
fn test_reoslve_test_repo_errors() {
  let cases = vec!{
    ("repo", "Repository must be in format 'username/repo' for mono-repo mode"),
    ("https://gitlab.com/user/repo", "Could not parse repository URL")
  };

  for (input, error) in cases {
    assert_eq!(
      resolve_test_repo(input),
      Err(error.to_string())
    )
  }
}

// create_mono_repo_cmakelists tests
#[test]
fn test_create_mono_repo_cmakelists_creates_file() {
  let tmp = std::env::temp_dir().join("star_setup_test_cmakelists");
  std::fs::create_dir_all(&tmp).ok();

  let repos = vec!["user/lib1".to_string(), "user/lib2".to_string()];
  create_mono_repo_cmakelists(&tmp, "user-testrepo", &repos, &mut sink()).unwrap();

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

  create_mono_repo_cmakelists(&tmp, "user-testrepo", &[], &mut sink()).unwrap();
  assert!(tmp.join("CMakeLists.txt").exists());

  std::fs::remove_dir_all(&tmp).ok();
}
