use super::common::{make_io, sink, MockRunner};
use star_setup::{
  cli::{
    resolve_with_config, Args, BuildFlags, ConfigFlags, ConnectionFlags, DiagnosticFlags,
    MonoRepoFlags, ProfileFlags,
  },
  commands::single_repo_mode,
  config::SetupConfig,
  ctx::RunCtx,
};
use tempfile::TempDir;

fn default_resolved() -> star_setup::cli::ResolvedArgs {
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
      no_build: true,
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

fn make_repo_fixture(base: &std::path::Path) {
  let repo_dir = base.join("user-repo");
  std::fs::create_dir_all(&repo_dir).unwrap();
  std::fs::write(repo_dir.join("CMakeLists.txt"), "").unwrap();
}

#[test]
fn test_single_repo_mode_updates_existing_repo() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved();
  make_repo_fixture(tmp.path());

  let mut input = b"y\n".as_ref();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();

  assert!(runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}

#[test]
fn test_single_repo_mode_skips_update_when_declined() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved();
  make_repo_fixture(tmp.path());

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();

  assert!(!runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}

#[test]
fn test_single_repo_mode_cleans_build_dir() {
  let tmp = TempDir::new().unwrap();
  let mut args = default_resolved();
  args.build.clean = true;
  make_repo_fixture(tmp.path());
  let build_dir = tmp.path().join("user-repo").join(&args.build.build_dir);
  std::fs::create_dir_all(&build_dir).unwrap();
  std::fs::write(build_dir.join("dummy.txt"), "old content").unwrap();

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx { io: make_io(&mut input, &mut output), runner: &mut runner };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();
  assert!(!build_dir.join("dummy.txt").exists());
  assert!(build_dir.exists());
}

#[test]
fn test_single_repo_mode_outputs_timing() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved();
  make_repo_fixture(tmp.path());

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: star_setup::ctx::IoCtx {
      input: &mut input,
      output: &mut output,
      verbose: false,
      timing: true,
    },
    runner: &mut runner,
  };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Total:"));
}
