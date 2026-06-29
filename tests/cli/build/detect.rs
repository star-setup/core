use star_setup::{
  cli::{detect_build_system, detect_mono_build_system, BuildSystem},
  ctx::{IoCtx, ProcessRunner, RunCtx},
};

fn create_cmake_fixture(path: &std::path::Path) {
  std::fs::write(path.join("CMakeLists.txt"), "").unwrap();
}

fn create_meson_fixture(path: &std::path::Path) {
  std::fs::write(path.join("meson.build"), "").unwrap();
}

fn with_detect_ctx<T, F>(input: &[u8], timing: bool, test_logic: F) -> (T, String)
where
  F: FnOnce(&std::path::Path, &mut RunCtx) -> T,
{
  let tmp = tempfile::TempDir::new().unwrap();
  let mut runner = ProcessRunner;
  let mut output = Vec::new();
  let mut input_slice = input;

  let result = {
    let mut ctx = RunCtx {
      io: IoCtx {
        input: &mut input_slice,
        output: &mut output,
      },
      flags: star_setup::ctx::RunFlags {
        verbose: false,
        timing,
        dry_run: false,
      },
      runner: &mut runner,
    };
    test_logic(tmp.path(), &mut ctx)
  };

  (result, String::from_utf8(output).unwrap())
}

#[test]
fn test_detect_build_system_none() {
  let (result, _) = with_detect_ctx(b"", false, detect_build_system);
  assert!(result.is_err());
}

#[test]
fn test_detect_build_system_cmake() {
  let (result, _) = with_detect_ctx(b"", false, |path, ctx| {
    create_cmake_fixture(path);
    detect_build_system(path, ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_meson() {
  let (result, _) = with_detect_ctx(b"", false, |path, ctx| {
    create_meson_fixture(path);
    detect_build_system(path, ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_both_picks_cmake() {
  let (result, _) = with_detect_ctx(b"1\n", false, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    detect_build_system(path, ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_both_picks_meson() {
  let (result, _) = with_detect_ctx(b"2\n", false, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    detect_build_system(path, ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_timing_output() {
  let ((), out) = with_detect_ctx(b"", true, |path, ctx| {
    create_cmake_fixture(path);
    detect_build_system(path, ctx).unwrap();
  });
  assert!(out.contains("[timing] Detect:"));
}

#[test]
fn test_detect_mono_build_system_none() {
  let (result, _) = with_detect_ctx(b"", false, |path, ctx| {
    detect_mono_build_system(&[path.to_path_buf()], ctx)
  });
  assert!(result.is_err());
}

#[test]
fn test_detect_mono_build_system_cmake() {
  let (result, _) = with_detect_ctx(b"", false, |path, ctx| {
    create_cmake_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_meson() {
  let (result, _) = with_detect_ctx(b"", false, |path, ctx| {
    create_meson_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_both_picks_cmake() {
  let (result, _) = with_detect_ctx(b"1\n", false, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_both_picks_meson() {
  let (result, _) = with_detect_ctx(b"2\n", false, |path, ctx| {
    create_cmake_fixture(path);
    create_meson_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx)
  });
  assert!(matches!(result.unwrap(), BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_timing_output() {
  let ((), out) = with_detect_ctx(b"", true, |path, ctx| {
    create_cmake_fixture(path);
    detect_mono_build_system(&[path.to_path_buf()], ctx).unwrap();
  });
  assert!(out.contains("[timing] Detect:"));
}
