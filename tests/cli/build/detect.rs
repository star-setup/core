use star_setup::{
  cli::build::{detect_build_system, detect_mono_build_system, BuildSystem},
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

#[test]
fn test_detect_build_system_none() {
  let dir = TempDir::new().unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_build_system(dir.path(), &mut ctx);
  assert!(result.is_err());
}

#[test]
fn test_detect_build_system_cmake() {
  let dir = cmake_dir();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_build_system(dir.path(), &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_meson() {
  let dir = meson_dir();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_build_system(dir.path(), &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"1\n".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_build_system(dir.path(), &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"2\n".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_build_system(dir.path(), &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_build_system_timing_output() {
  let dir = cmake_dir();
  let mut output = Vec::new();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut output,
      verbose: false,
      timing: true,
    },
    runner: &mut runner,
  };
  detect_build_system(dir.path(), &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Detect:"));
}

#[test]
fn test_detect_mono_build_system_cmake() {
  let dir = cmake_dir();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_meson() {
  let dir = meson_dir();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_none() {
  let dir = TempDir::new().unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx);
  assert!(result.is_err());
}

#[test]
fn test_detect_mono_build_system_both_picks_cmake() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"1\n".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Cmake));
}

#[test]
fn test_detect_mono_build_system_both_picks_meson() {
  let dir = cmake_dir();
  std::fs::write(dir.path().join("meson.build"), "").unwrap();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"2\n".as_ref(),
      output: &mut Vec::new(),
      verbose: false,
      timing: false,
    },
    runner: &mut runner,
  };
  let result = detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx).unwrap();
  assert!(matches!(result, BuildSystem::Meson));
}

#[test]
fn test_detect_mono_build_system_timing_output() {
  let dir = cmake_dir();
  let mut output = Vec::new();
  let mut runner = ProcessRunner { verbose: false };
  let mut ctx = RunCtx {
    io: IoCtx {
      input: &mut b"".as_ref(),
      output: &mut output,
      verbose: false,
      timing: true,
    },
    runner: &mut runner,
  };
  detect_mono_build_system(&[dir.path().to_path_buf()], &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Detect:"));
}
