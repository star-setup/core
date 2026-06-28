
use star_setup::{
  cli::{detect_build_system, detect_mono_build_system, BuildSystem},
  ctx::{IoCtx, ProcessRunner, RunCtx},
};
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

fn run_test<T, F>(input: &[u8], timing: bool, test_logic: F) -> (T, Vec<u8>)
where
  F: FnOnce(&mut RunCtx) -> T,
{
  let mut runner = ProcessRunner;
  let mut output = Vec::new();
  let mut input_slice = input;

  let result = {
    let mut ctx = RunCtx {
      io: IoCtx {
        input: &mut input_slice,
        output: &mut output,
        verbose: false,
        timing,
        dry_run: false,
      },
      runner: &mut runner,
    };

    test_logic(&mut ctx)
  };

  (result, output)
}

#[test]
fn test_detect_build_system_none() {
  let dir = TempDir::new().unwrap();
  let (result, _) = run_test(b"", false, |ctx| detect_build_system(dir.path(), ctx));
  assert!(result.is_err());
}

#[test]
fn test_detect_build_system_cmake() {
  let dir = cmake_dir();
  let (result, _) = run_test(b"", false, |ctx| detect_build_system(dir.path(), ctx));
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_meson() {
  let dir = meson_dir();
  let (result, _) = run_test(b"", false, |ctx| detect_build_system(dir.path(), ctx));
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let (result, _) = run_test(b"1\n", false, |ctx| detect_build_system(dir.path(), ctx));
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let (result, _) = run_test(b"2\n", false, |ctx| detect_build_system(dir.path(), ctx));
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_timing_output() {
  let dir = cmake_dir();
  let ((), output) = run_test(b"", true, |ctx| {
    detect_build_system(dir.path(), ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Detect:"));
}

#[test]
fn test_detect_mono_build_system_none() {
  let dir = TempDir::new().unwrap();
  let (result, _) = run_test(b"", false, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx)
  });
  assert!(result.is_err());
}

#[test]
fn test_detect_mono_build_system_cmake() {
  let dir = cmake_dir();
  let (result, _) = run_test(b"", false, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_meson() {
  let dir = meson_dir();
  let (result, _) = run_test(b"", false, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let (result, _) = run_test(b"1\n", false, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let (result, _) = run_test(b"2\n", false, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_timing_output() {
  let dir = cmake_dir();
  let ((), output) = run_test(b"", true, |ctx| {
    detect_mono_build_system(&[dir.path().to_path_buf()], ctx).unwrap();
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("[timing] Detect:"));
}
