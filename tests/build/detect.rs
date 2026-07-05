use crate::common::with_ctx_input;
use star_setup::{
  build::{detect_build_system, detect_mono_build_system, BuildSystem},
  ctx::ProcessRunner,
};
use std::{fs::write, path::Path};

/* =====     HELPERS     ===== */
fn create_cmake_fixture(path: &Path) {
  write(path.join("CMakeLists.txt"), "").unwrap();
}

fn create_meson_fixture(path: &Path) {
  write(path.join("meson.build"), "").unwrap();
}

fn create_npm_fixture(path: &Path) {
  write(path.join("package.json"), "{}").unwrap();
}

/* =====     DETECT_BUILD_SYSTEM     ===== */
#[test]
fn test_detect_build_system_none() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    assert!(detect_build_system(path, ctx).is_err());
  });
}

#[test]
fn test_detect_build_system_cmake() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Cmake
    ));
  });
}

#[test]
fn test_detect_build_system_meson() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_meson_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Meson
    ));
  });
}

#[test]
fn test_detect_build_system_both_picks_cmake() {
  with_ctx_input(b"1\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Cmake
    ));
  });
}

#[test]
fn test_detect_build_system_both_picks_meson() {
  with_ctx_input(b"2\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Meson
    ));
  });
}

#[test]
fn test_detect_build_system_timing_output() {
  let (_, output) = with_ctx_input(b"", ProcessRunner, |path, ctx| {
    ctx.flags.timing = true;
    create_cmake_fixture(path);
    detect_build_system(path, ctx).unwrap();
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("[timing] Scanned directory:"));
}

#[test]
fn test_detect_build_system_npm() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_npm_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Npm
    ));
  });
}

#[test]
fn test_detect_build_system_cmake_and_npm_picks_npm() {
  with_ctx_input(b"2\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_npm_fixture(path);
    assert!(matches!(
      detect_build_system(path, ctx).unwrap(),
      BuildSystem::Npm
    ));
  });
}

/* =====     DETECT_MONO_BUILD_SYSTEM     ===== */
#[test]
fn test_detect_mono_build_system_none() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    assert!(detect_mono_build_system(&[path.to_path_buf()], ctx).is_err());
  });
}

#[test]
fn test_detect_mono_build_system_cmake() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Cmake
    ));
  });
}

#[test]
fn test_detect_mono_build_system_meson() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_meson_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Meson
    ));
  });
}

#[test]
fn test_detect_mono_build_system_both_picks_cmake() {
  with_ctx_input(b"1\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Cmake
    ));
  });
}

#[test]
fn test_detect_mono_build_system_both_picks_meson() {
  with_ctx_input(b"2\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Meson
    ));
  });
}

#[test]
fn test_detect_mono_build_system_timing_output() {
  let (_, output) = with_ctx_input(b"", ProcessRunner, |path, ctx| {
    ctx.flags.timing = true;
    create_cmake_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap();
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("[timing] Scanned directories:"));
}

#[test]
fn test_detect_mono_build_system_npm() {
  with_ctx_input(b"", ProcessRunner, |path, ctx| {
    create_npm_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Npm
    ));
  });
}

#[test]
fn test_detect_mono_build_system_cmake_and_npm_picks_npm() {
  with_ctx_input(b"2\n", ProcessRunner, |path, ctx| {
    create_cmake_fixture(path);
    create_npm_fixture(path);
    assert!(matches!(
      detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap(),
      BuildSystem::Npm
    ));
  });
}
