use crate::common::{default_resolved_mono, empty_input, make_io, MockRunner};
use star_setup::{
  commands::mono_repo_mode,
  config::SetupConfig,
  ctx::{DryRunRunner, RunCtx, Runner},
};

fn run_mode_test<R, F>(mut runner: R, test_logic: F) -> (R, Vec<u8>)
where
  R: Runner,
  F: FnOnce(&std::path::Path, &mut RunCtx<'_, '_>),
{
  let tmp = tempfile::TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = Vec::new();

  {
    let mut ctx = RunCtx {
      io: make_io(&mut input, &mut output),
      runner: &mut runner,
    };
    test_logic(tmp.path(), &mut ctx);
  }

  (runner, output)
}

fn make_cmake_repo(repos_path: &std::path::Path, name: &str) {
  let dir = repos_path.join(name);
  std::fs::create_dir_all(&dir).unwrap();
  std::fs::write(dir.join("CMakeLists.txt"), "").unwrap();
}

#[test]
fn test_mono_repo_mode_clones_and_configures() {
  let args = default_resolved_mono(vec!["user/lib1".to_string()]);

  let (_, output) = run_mode_test(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join(&args.mono.mono_dir).join("repos");
    std::fs::create_dir_all(&repos_path).unwrap();
    make_cmake_repo(&repos_path, "user-lib1");
    make_cmake_repo(&repos_path, "user-test-repo");

    mono_repo_mode(&args, &SetupConfig::new(), tmp_path, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Setup complete"));
  assert!(out.contains("Total repositories:"));
}

#[test]
fn test_mono_repo_mode_dry_run_makes_no_fs_changes() {
  let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
  args.diagnostic.dry_run = true;

  run_mode_test(DryRunRunner, |tmp_path, ctx| {
    ctx.io.dry_run = true;

    mono_repo_mode(&args, &SetupConfig::new(), tmp_path, ctx).unwrap();

    assert!(std::fs::read_dir(tmp_path).unwrap().next().is_none());
  });
}

#[test]
fn test_mono_repo_mode_with_build_system_flag() {
  let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
  args.build.build_system = Some(star_setup::cli::BuildSystem::Cmake);

  let (runner, _) = run_mode_test(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join(&args.mono.mono_dir).join("repos");
    std::fs::create_dir_all(&repos_path).unwrap();
    make_cmake_repo(&repos_path, "user-lib1");
    make_cmake_repo(&repos_path, "user-test-repo");

    mono_repo_mode(&args, &SetupConfig::new(), tmp_path, ctx).unwrap();
  });

  assert!(runner.calls.iter().any(|(cmd, _)| cmd[0] == "cmake"));
}
