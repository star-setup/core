use crate::common::{make_flags, with_io_dir};
use star_setup::commands::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild};

// create_mono_repo_cmakelists tests
#[test]
fn test_create_mono_repo_cmakelists_creates_file() {
  with_io_dir(|tmp_path, io| {
    let repos = vec![
      "user-testrepo".to_string(),
      "user/lib1".to_string(),
      "user/lib2".to_string(),
    ];
    create_mono_repo_cmakelists(tmp_path, &repos, io, make_flags()).unwrap();

    let cmake_file = tmp_path.join("CMakeLists.txt");
    assert!(cmake_file.exists());

    let content = std::fs::read_to_string(&cmake_file).unwrap();
    assert!(content.contains("user-testrepo"));
    assert!(content.contains("user-lib1"));
    assert!(content.contains("user-lib2"));
  });
}

#[test]
fn test_create_mono_repo_cmakelists_empty_repos() {
  with_io_dir(|tmp_path, io| {
    let repos = vec!["user-testrepo".to_string()];
    create_mono_repo_cmakelists(tmp_path, &repos, io, make_flags()).unwrap();
    assert!(tmp_path.join("CMakeLists.txt").exists());
  });
}

// create_mono_repo_mesonbuild tests
#[test]
fn test_create_mono_repo_mesonbuild_creates_file() {
  with_io_dir(|tmp_path, io| {
    let repos = vec![
      "user-testrepo".to_string(),
      "user/lib1".to_string(),
      "user/lib2".to_string(),
    ];
    create_mono_repo_mesonbuild(tmp_path, &repos, io, make_flags()).unwrap();

    let meson_file = tmp_path.join("meson.build");
    assert!(meson_file.exists());

    let content = std::fs::read_to_string(&meson_file).unwrap();
    assert!(content.contains("user-testrepo"));
    assert!(content.contains("user-lib1"));
    assert!(content.contains("user-lib2"));
  });
}

#[test]
fn test_create_mono_repo_mesonbuild_empty_repos() {
  with_io_dir(|tmp_path, io| {
    let repos = vec!["user-testrepo".to_string()];
    create_mono_repo_mesonbuild(tmp_path, &repos, io, make_flags()).unwrap();
    assert!(tmp_path.join("meson.build").exists());
  });
}

#[test]
fn test_create_mono_repo_package_json_creates_file() {
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
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
    star_setup::commands::create_mono_repo_package_json(
      tmp_path,
      &repos_path,
      &repos,
      io,
      make_flags(),
    )
    .unwrap();

    let content = std::fs::read_to_string(tmp_path.join("package.json")).unwrap();
    assert!(content.contains("workspaces"));
    assert!(content.contains("repos/user-lib1"));
    assert!(content.contains("repos/user-lib2"));
    assert!(content.contains("overrides"));
    assert!(content.contains("@user/lib1"));
    assert!(content.contains("@user/lib2"));
  });
}
