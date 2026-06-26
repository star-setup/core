use super::common::{default_resolved, make_io, sink, MockRunner};
use star_setup::{
  commands::single_repo_mode,
  ctx::{DryRunRunner, RunCtx},
};
use tempfile::TempDir;

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
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

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
      dry_run: false,
    },
    runner: &mut runner,
  };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Total:"));
}

#[test]
fn test_single_repo_mode_dry_run_makes_no_fs_changes() {
  let tmp = TempDir::new().unwrap();
  let mut args = default_resolved();
  args.diagnostic.dry_run = true;

  let mut input = b"".as_ref();
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

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();
  assert!(std::fs::read_dir(tmp.path()).unwrap().next().is_none());
}

#[test]
fn test_single_repo_mode_dry_run_clean_prints_would_remove() {
  let tmp = TempDir::new().unwrap();
  let mut args = default_resolved();
  args.diagnostic.dry_run = true;
  args.build.clean = true;

  let mut input = b"".as_ref();
  let mut output = Vec::new();
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

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Would remove directory:"));
  assert!(std::fs::read_dir(tmp.path()).unwrap().next().is_none());
}

#[test]
fn test_single_repo_mode_with_build_system_flag() {
  let tmp = TempDir::new().unwrap();
  let mut args = default_resolved();
  args.build.build_system = Some(star_setup::cli::BuildSystem::Cmake);
  make_repo_fixture(tmp.path());

  let mut input = b"n\n".as_ref();
  let mut output = sink();
  let mut runner = MockRunner::new();
  let mut ctx = RunCtx {
    io: make_io(&mut input, &mut output),
    runner: &mut runner,
  };

  single_repo_mode(&args, tmp.path(), &mut ctx).unwrap();

  assert!(runner.calls.iter().any(|(cmd, _)| cmd[0] == "cmake"));
}
