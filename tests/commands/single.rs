use crate::common::{default_resolved, with_ctx_input, MockRunner};
use star_setup::{cli::BuildSystem, commands::single_repo_mode, ctx::DryRunRunner};

fn make_repo_fixture(base: &std::path::Path) {
  let repo_dir = base.join("user-repo");
  std::fs::create_dir_all(&repo_dir).unwrap();
  std::fs::write(repo_dir.join("CMakeLists.txt"), "").unwrap();
}

#[test]
fn test_single_repo_mode_updates_existing_repo() {
  let (runner, _) = with_ctx_input(b"y\n", MockRunner::new(), |tmp_path, ctx| {
    make_repo_fixture(tmp_path);
    single_repo_mode(&default_resolved(), tmp_path, ctx).unwrap();
  });
  assert!(runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}

#[test]
fn test_single_repo_mode_skips_update_when_declined() {
  let (runner, _) = with_ctx_input(b"n\n", MockRunner::new(), |tmp_path, ctx| {
    make_repo_fixture(tmp_path);
    single_repo_mode(&default_resolved(), tmp_path, ctx).unwrap();
  });
  assert!(!runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}

#[test]
fn test_single_repo_mode_cleans_build_dir() {
  let mut args = default_resolved();
  args.build.clean = true;

  with_ctx_input(b"n\n", MockRunner::new(), |tmp_path, ctx| {
    make_repo_fixture(tmp_path);
    let build_dir = tmp_path.join("user-repo").join(&args.build.build_dir);
    std::fs::create_dir_all(&build_dir).unwrap();
    std::fs::write(build_dir.join("dummy.txt"), "old content").unwrap();
    single_repo_mode(&args, tmp_path, ctx).unwrap();
    assert!(!build_dir.join("dummy.txt").exists());
    assert!(build_dir.exists());
  });
}

#[test]
fn test_single_repo_mode_outputs_timing() {
  let (_, output) = with_ctx_input(b"n\n", MockRunner::new(), |tmp_path, ctx| {
    make_repo_fixture(tmp_path);
    ctx.flags.timing = true;
    single_repo_mode(&default_resolved(), tmp_path, ctx).unwrap();
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("[timing] Total:"));
}

#[test]
fn test_single_repo_mode_dry_run_makes_no_fs_changes() {
  let mut args = default_resolved();
  args.diagnostic.dry_run = true;

  with_ctx_input(b"", DryRunRunner, |tmp_path, ctx| {
    ctx.flags.dry_run = true;
    single_repo_mode(&args, tmp_path, ctx).unwrap();
    assert!(std::fs::read_dir(tmp_path).unwrap().next().is_none());
  });
}

#[test]
fn test_single_repo_mode_dry_run_clean_prints_would_remove() {
  let mut args = default_resolved();
  args.diagnostic.dry_run = true;
  args.build.clean = true;
  args.build.build_system = Some(BuildSystem::Cmake);

  let (_, output) = with_ctx_input(b"", DryRunRunner, |tmp_path, ctx| {
    ctx.flags.dry_run = true;
    single_repo_mode(&args, tmp_path, ctx).unwrap();
    assert!(std::fs::read_dir(tmp_path).unwrap().next().is_none());
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("Would remove directory:"));
}

#[test]
fn test_single_repo_mode_with_build_system_flag() {
  let mut args = default_resolved();
  args.build.build_system = Some(BuildSystem::Cmake);

  let (runner, _) = with_ctx_input(b"n\n", MockRunner::new(), |tmp_path, ctx| {
    make_repo_fixture(tmp_path);
    single_repo_mode(&args, tmp_path, ctx).unwrap();
  });
  assert!(runner.calls.iter().any(|(cmd, _)| cmd[0] == "cmake"));
}
