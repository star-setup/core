use crate::common::{default_resolved_mono, with_ctx, with_ctx_runner, MockRunner};
use star_setup::{
  build::BuildSystem, commands::mono_repo_mode, config::Config, ctx::DryRunRunner, profile::Profile,
};
use std::{
  collections::BTreeMap,
  fs::{create_dir_all, read_dir, write},
  path::Path,
};

fn make_cmake_repo(repos_path: &Path, name: &str) {
  let dir = repos_path.join(name);
  create_dir_all(dir.join(".git")).unwrap();
  write(dir.join("CMakeLists.txt"), "").unwrap();
}

#[test]
fn test_mono_repo_mode_clones_and_configures() {
  let args = default_resolved_mono(vec!["user/lib1".to_string()]);

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join(&args.mono.mono_dir).join("repos");
    create_dir_all(&repos_path).unwrap();
    make_cmake_repo(&repos_path, "user-lib1");
    make_cmake_repo(&repos_path, "user-test-repo");

    mono_repo_mode(&args, &Config::new(), tmp_path, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Setup complete"));
  assert!(out.contains("Total repositories:"));
}

#[test]
fn test_mono_repo_mode_dry_run_makes_no_fs_changes() {
  let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
  args.diagnostic.dry_run = true;

  with_ctx(DryRunRunner, |tmp_path, ctx| {
    ctx.flags.dry_run = true;

    mono_repo_mode(&args, &Config::new(), tmp_path, ctx).unwrap();

    assert!(read_dir(tmp_path).unwrap().next().is_none());
  });
}

#[test]
fn test_mono_repo_mode_dry_run_with_build_system_makes_no_fs_changes() {
  for bs in [BuildSystem::Npm, BuildSystem::Cmake, BuildSystem::Meson] {
    let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
    args.diagnostic.dry_run = true;
    args.build.build_system = Some(bs);
    args.build.watch = true;

    with_ctx(DryRunRunner, |tmp_path, ctx| {
      ctx.flags.dry_run = true;

      mono_repo_mode(&args, &Config::new(), tmp_path, ctx).unwrap();

      assert!(
        read_dir(tmp_path).unwrap().next().is_none(),
        "{bs:?} dry-run wrote to disk"
      );
    });
  }
}

#[test]
fn test_mono_repo_mode_with_build_system_flag() {
  let mut args = default_resolved_mono(vec!["user/lib1".to_string()]);
  args.build.build_system = Some(BuildSystem::Cmake);

  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join(&args.mono.mono_dir).join("repos");
    create_dir_all(&repos_path).unwrap();
    make_cmake_repo(&repos_path, "user-lib1");
    make_cmake_repo(&repos_path, "user-test-repo");

    mono_repo_mode(&args, &Config::new(), tmp_path, ctx).unwrap();
  });

  assert!(runner.calls.iter().any(|(cmd, _)| cmd[0] == "cmake"));
}

#[test]
fn test_mono_repo_mode_multiple_test_repos_opens_all() {
  let mut config = Config::new();
  config.profiles.insert(
    "multi".to_string(),
    Profile {
      test_repos: BTreeMap::from([
        ("game1".to_string(), "user/game1".to_string()),
        ("game2".to_string(), "user/game2".to_string()),
      ]),
      deps: BTreeMap::new(),
    },
  );
  let mut args = default_resolved_mono(vec![]);
  args.repo = None;
  args.mono.profile = Some("multi".to_string());

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join(&args.mono.mono_dir).join("repos");
    create_dir_all(&repos_path).unwrap();
    make_cmake_repo(&repos_path, "user-game1");
    make_cmake_repo(&repos_path, "user-game2");
    mono_repo_mode(&args, &config, tmp_path, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Total repositories: 2"));
}
