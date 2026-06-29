use super::common::{default_resolved_with_no_build, with_runner_ctx, MockRunner};
use star_setup::{
  cli::BuildSystem,
  commands::{build_project, cmake_build, meson_build},
};

#[test]
fn test_cmake_build_configure_only() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    cmake_build(&args, tmp_path, false, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 1);
  assert!(runner.calls[0].0.contains(&"cmake".to_string()));
}

#[test]
fn test_cmake_build_with_build_step() {
  let args = default_resolved_with_no_build(false);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    cmake_build(&args, tmp_path, false, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls[1].0.contains(&"--build".to_string()));
}

#[test]
fn test_cmake_build_mono_flag() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    cmake_build(&args, tmp_path, true, ctx).unwrap();
  });
  assert!(runner.calls[0].0.contains(&"-DBUILD_LOCAL=ON".to_string()));
}

#[test]
fn test_meson_build_configure_only() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    meson_build(&args, tmp_path, tmp_path, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 1);
  assert!(runner.calls[0].0.contains(&"meson".to_string()));
  assert!(runner.calls[0].0.contains(&"setup".to_string()));
}

#[test]
fn test_meson_build_with_build_step() {
  let args = default_resolved_with_no_build(false);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    meson_build(&args, tmp_path, tmp_path, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls[1].0.contains(&"compile".to_string()));
}

#[test]
fn test_build_project_dispatches_cmake() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    build_project(&args, tmp_path, tmp_path, BuildSystem::Cmake, false, ctx).unwrap();
  });
  assert!(runner.calls[0].0.contains(&"cmake".to_string()));
}

#[test]
fn test_build_project_dispatches_meson() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    build_project(&args, tmp_path, tmp_path, BuildSystem::Meson, false, ctx).unwrap();
  });
  assert!(runner.calls[0].0.contains(&"meson".to_string()));
}

#[test]
fn test_npm_build_install_only() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    star_setup::commands::npm_build(&args, tmp_path, false, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 1);
  assert!(runner.calls[0].0.contains(&"install".to_string()));
}

#[test]
fn test_npm_build_with_build_step() {
  let args = default_resolved_with_no_build(false);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    star_setup::commands::npm_build(&args, tmp_path, false, ctx).unwrap();
  });
  assert_eq!(runner.calls.len(), 2);
  assert!(runner.calls[1].0.contains(&"build".to_string()));
}

#[test]
fn test_build_project_dispatches_npm() {
  let args = default_resolved_with_no_build(true);
  let runner = with_runner_ctx(MockRunner::new(), |tmp_path, ctx| {
    build_project(&args, tmp_path, tmp_path, BuildSystem::Npm, false, ctx).unwrap();
  });
  assert!(runner.calls[0].0.contains(&"install".to_string()));
}
