use super::super::common::{empty_input, make_io, sink};
use star_setup::commands::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild};

// create_mono_repo_cmakelists tests
#[test]
fn test_create_mono_repo_cmakelists_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();

  let repos = vec![
    "user-testrepo".to_string(),
    "user/lib1".to_string(),
    "user/lib2".to_string(),
  ];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  create_mono_repo_cmakelists(tmp.path(), &repos, &mut io).unwrap();

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
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  create_mono_repo_cmakelists(tmp.path(), &repos, &mut io).unwrap();
  assert!(tmp.path().join("CMakeLists.txt").exists());
}

// create_mono_repo_mesonbuild tests
#[test]
fn test_create_mono_repo_mesonbuild_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();
  let repos = vec![
    "user-testrepo".to_string(),
    "user/lib1".to_string(),
    "user/lib2".to_string(),
  ];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  create_mono_repo_mesonbuild(tmp.path(), &repos, &mut io).unwrap();
  let meson_file = tmp.path().join("meson.build");
  assert!(meson_file.exists());
  let content = std::fs::read_to_string(&meson_file).unwrap();
  assert!(content.contains("user-testrepo"));
  assert!(content.contains("user-lib1"));
  assert!(content.contains("user-lib2"));
}

#[test]
fn test_create_mono_repo_mesonbuild_empty_repos() {
  let tmp = tempfile::TempDir::new().unwrap();
  let repos = vec!["user-testrepo".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  create_mono_repo_mesonbuild(tmp.path(), &repos, &mut io).unwrap();
  assert!(tmp.path().join("meson.build").exists());
}

#[test]
fn test_create_mono_repo_package_json_creates_file() {
  let tmp = tempfile::TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");

  // Create fake lib package.json files
  std::fs::create_dir_all(repos_path.join("user-lib1")).unwrap();
  std::fs::create_dir_all(repos_path.join("user-lib2")).unwrap();
  std::fs::write(
    repos_path.join("user-lib1").join("package.json"),
    r#"{"name": "@user/lib1"}"#,
  )
  .unwrap();
  std::fs::write(
    repos_path.join("user-lib2").join("package.json"),
    r#"{"name": "@user/lib2"}"#,
  )
  .unwrap();

  let repos = vec![
    "user/game".to_string(),
    "user/lib1".to_string(),
    "user/lib2".to_string(),
  ];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  star_setup::commands::create_mono_repo_package_json(tmp.path(), &repos_path, &repos, &mut io)
    .unwrap();
  let pkg = tmp.path().join("package.json");
  assert!(pkg.exists());
  let content = std::fs::read_to_string(&pkg).unwrap();
  assert!(content.contains("workspaces"));
  assert!(content.contains("repos/user-lib1"));
  assert!(content.contains("repos/user-lib2"));
  assert!(content.contains("overrides"));
  assert!(content.contains("@user/lib1"));
  assert!(content.contains("@user/lib2"));
}
