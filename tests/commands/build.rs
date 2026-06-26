use super::common::{empty_input, make_io, sink, MockRunner};
use star_setup::{
  cli::{
    resolve_with_config, Args, BuildFlags, BuildSystem, ConfigFlags, ConnectionFlags,
    DiagnosticFlags, MonoRepoFlags, ProfileFlags,
  },
  commands::{build_project, cmake_build, meson_build},
  config::SetupConfig,
  ctx::RunCtx,
};
use tempfile::TempDir;

fn default_resolved(no_build: bool) -> star_setup::cli::ResolvedArgs {
  let args = Args {
    repo: Some("user/repo".to_string()),
    yes: false,
    diagnostic: DiagnosticFlags { timing: false },
    connection: ConnectionFlags {
      ssh: false,
      https: false,
      verbose: false,
      no_verbose: false,
    },
    build: BuildFlags {
      build_type: None,
      build_dir: None,
      no_build,
      build: false,
      clean: false,
      no_clean: false,
      cmake_flags: vec![],
      meson_flags: vec![],
    },
    mono: MonoRepoFlags {
      mono_repo: false,
      mono_dir: None,
      repos: None,
      profile: None,
    },
    config: ConfigFlags {
      init_config: false,
      config_name: None,
      config_add: None,
      config_remove: None,
      list_configs: false,
    },
    profile: ProfileFlags {
      profile_add: None,
      profile_remove: None,
      list_profiles: false,
    },
  };
  resolve_with_config(args, &SetupConfig::new()).unwrap()
}

#[test]
fn test_cmake_build_configure_only() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved(true);
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
  let args = default_resolved(false);
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
  let args = default_resolved(true);
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
  let args = default_resolved(true);
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
  let args = default_resolved(false);
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
  let args = default_resolved(true);
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
  let args = default_resolved(true);
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
