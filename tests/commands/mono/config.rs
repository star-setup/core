use crate::common::with_io_dir;
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
    create_mono_repo_cmakelists(tmp_path, &repos, io).unwrap();

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
    create_mono_repo_cmakelists(tmp_path, &repos, io).unwrap();
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
    create_mono_repo_mesonbuild(tmp_path, &repos, io).unwrap();

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
    create_mono_repo_mesonbuild(tmp_path, &repos, io).unwrap();
    assert!(tmp_path.join("meson.build").exists());
  });
}
