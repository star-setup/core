use star_setup::cli::build::{detect_build_system, detect_mono_build_system, BuildSystem};
use tempfile::TempDir;

fn cmake_dir() -> TempDir {
  let tmp = TempDir::new().unwrap();
  std::fs::write(tmp.path().join("CMakeLists.txt"), "").unwrap();
  tmp
}

fn meson_dir() -> TempDir {
  let tmp = TempDir::new().unwrap();
  std::fs::write(tmp.path().join("meson.build"), "").unwrap();
  tmp
}

#[test]
fn test_detect_build_system_cmake() {
  let dir = cmake_dir();
  let result = detect_build_system(dir.path(), &mut b"".as_ref(), &mut Vec::new()).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_meson() {
  let dir = meson_dir();
  let result = detect_build_system(dir.path(), &mut b"".as_ref(), &mut Vec::new()).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_none() {
  let dir = TempDir::new().unwrap();
  let result = detect_build_system(dir.path(), &mut b"".as_ref(), &mut Vec::new());
  assert!(result.is_err());
}

#[test]
fn test_detect_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let result = detect_build_system(dir.path(), &mut b"1\n".as_ref(), &mut Vec::new()).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let result = detect_build_system(dir.path(), &mut b"2\n".as_ref(), &mut Vec::new()).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_cmake() {
  let dir = cmake_dir();
  let result = detect_mono_build_system(
    &[dir.path().to_path_buf()],
    &mut b"".as_ref(),
    &mut Vec::new(),
  )
  .unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_meson() {
  let dir = meson_dir();
  let result = detect_mono_build_system(
    &[dir.path().to_path_buf()],
    &mut b"".as_ref(),
    &mut Vec::new(),
  )
  .unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_none() {
  let dir = TempDir::new().unwrap();
  let result = detect_mono_build_system(
    &[dir.path().to_path_buf()],
    &mut b"".as_ref(),
    &mut Vec::new(),
  );
  assert!(result.is_err());
}

#[test]
fn test_detect_mono_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let result = detect_mono_build_system(
    &[dir.path().to_path_buf()],
    &mut b"1\n".as_ref(),
    &mut Vec::new(),
  )
  .unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let result = detect_mono_build_system(
    &[dir.path().to_path_buf()],
    &mut b"2\n".as_ref(),
    &mut Vec::new(),
  )
  .unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}
