use crate::common::{default_resolved_mono, empty_input, make_io, sink, MockRunner};
use star_setup::{
  commands::mono_repo_mode,
  config::SetupConfig,
  ctx::{DryRunRunner, RunCtx},
};
use tempfile::TempDir;

fn make_cmake_repo(repos_path: &std::path::Path, name: &str) {
  let dir = repos_path.join(name);
  std::fs::create_dir_all(&dir).unwrap();
  std::fs::write(dir.join("CMakeLists.txt"), "").unwrap();
}

#[test]
fn test_mono_repo_mode_clones_and_configures() {
  let tmp = TempDir::new().unwrap();
  let args = default_resolved_mono(vec!["user/lib1".to_string()]);

  let repos_path = tmp.path().join(&args.mono.mono_dir).join("repos");
  std::fs::create_dir_all(&repos_path).unwrap();
  make_cmake_repo(&repos_path, "user-lib1");
  make_cmake_repo(&repos_path, "user-test-repo");

  let mut input = empty_input();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  mono_repo_mode(&args, &SetupConfig::new(), tmp.path(), &mut ctx).unwrap();

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Setup complete"));
  assert!(out.contains("Total repositories:"));
}

#[test]
fn test_mono_repo_mode_dry_run_makes_no_fs_changes() {
  let tmp = TempDir::new().unwrap();
  let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
  args.diagnostic.dry_run = true;

  let mut input = empty_input();
  let mut output = sink();
  let mut runner = DryRunRunner;
  let mut ctx = RunCtx {
    io: star_setup::ctx::IoCtx {
      input: &mut input,
      output: &mut output,
      verbose: false,
      timing: false,
      dry_run: true,
    },
    runner: &mut runner,
  };

  mono_repo_mode(&args, &SetupConfig::new(), tmp.path(), &mut ctx).unwrap();

  assert!(std::fs::read_dir(tmp.path()).unwrap().next().is_none());
}
