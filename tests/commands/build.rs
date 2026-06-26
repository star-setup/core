use super::common::{default_resolved_with_no_build, empty_input, make_io, sink, MockRunner};
use star_setup::{
  cli::BuildSystem,
  commands::{build_project, cmake_build, meson_build},
  ctx::RunCtx,
};
use tempfile::TempDir;

#[test]
fn test_cmake_build_configure_only() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(true);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  cmake_build(&args, tmp.path(), false, &mut ctx).unwrap();

  assert_eq!(runner.calls.len(), 1);
  assert!(runner.calls[0].0.contains(&"cmake".to_string()));
}

#[test]
fn test_cmake_build_with_build_step() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(false);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  cmake_build(&args, tmp.path(), false, &mut ctx).unwrap();

  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls[1].0.contains(&"--build".to_string()));
}

#[test]
fn test_cmake_build_mono_flag() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(true);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  cmake_build(&args, tmp.path(), true, &mut ctx).unwrap();

  assert!(runner.calls[0].0.contains(&"-DBUILD_LOCAL=ON".to_string()));
}

#[test]
fn test_meson_build_configure_only() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(true);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  meson_build(&args, tmp.path(), tmp.path(), &mut ctx).unwrap();

  assert_eq!(runner.calls.len(), 1);
  assert!(runner.calls[0].0.contains(&"meson".to_string()));
  assert!(runner.calls[0].0.contains(&"setup".to_string()));
}

#[test]
fn test_meson_build_with_build_step() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(false);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  meson_build(&args, tmp.path(), tmp.path(), &mut ctx).unwrap();

  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls[1].0.contains(&"compile".to_string()));
}

#[test]
fn test_build_project_dispatches_cmake() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(true);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  build_project(
    &args,
    tmp.path(),
    tmp.path(),
    BuildSystem::Cmake,
    false,
    &mut ctx,
  )
  .unwrap();

  assert!(runner.calls[0].0.contains(&"cmake".to_string()));
}

#[test]
fn test_build_project_dispatches_meson() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_with_no_build(true);
  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  build_project(
    &args,
    tmp.path(),
    tmp.path(),
    BuildSystem::Meson,
    false,
    &mut ctx,
  )
  .unwrap();

  assert!(runner.calls[0].0.contains(&"meson".to_string()));
}
