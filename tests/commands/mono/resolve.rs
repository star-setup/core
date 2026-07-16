use crate::common::{default_resolved, with_ctx, MockRunner};
use star_setup::{
  build::{generate_mono_config, BuildSystem},
  commands::{resolve_dep_repos_for_mono, resolve_test_repo, resolve_test_repos_for_mono},
  profile::Profile,
};
use std::{
  collections::BTreeMap,
  fs::{create_dir_all, read_to_string, write},
  slice::from_ref,
};

/* =====     RESOLVE_TEST_REPO     ===== */
#[test]
fn test_resolve_test_repo() {
  let cases = [
    "user/repo",
    "user/repo/",
    "https://github.com/user/repo",
    "https://github.com/user/repo.git",
    "git@github.com:user/repo.git",
    "git@github.com:user/repo",
  ];
  for input in cases {
    assert_eq!(
      resolve_test_repo(input),
      Ok("user/repo".to_string()),
      "Failed for input: {input}"
    );
  }
}

#[test]
fn test_resolve_test_repo_errors() {
  let cases = vec![
    (
      "repo",
      "Repository must be in format 'username/repo' for mono-repo mode",
    ),
    (
      "https://gitlab.com/user/repo",
      "Could not parse repository URL",
    ),
    (
      "git@github.com:owner",
      "Repository URL missing repository name",
    ),
  ];
  for (input, error) in cases {
    assert_eq!(resolve_test_repo(input), Err(error.to_string()));
  }
}

/* =====     RESOLVE_REPOS_FOR_MONO     ===== */
#[test]
fn test_resolve_test_repos_for_mono_errors_when_no_repo() {
  let mut args = default_resolved();
  args.repo = None;
  assert!(resolve_test_repos_for_mono(&args, None).is_err());
}

#[test]
fn test_resolve_test_repos_for_mono_from_profile() {
  let profile = Profile {
    test_repos: BTreeMap::from([
      ("game1".to_string(), "user/game1".to_string()),
      ("game2".to_string(), "user/game2".to_string()),
    ]),
    deps: BTreeMap::new(),
  };
  let args = default_resolved();
  assert_eq!(
    resolve_test_repos_for_mono(&args, Some(&profile)),
    Ok(vec!["user/game1".to_string(), "user/game2".to_string()])
  );
}

#[test]
fn test_resolve_repos_for_mono_with_explicit_repos() {
  let mut args = default_resolved();
  args.mono.repos = Some(vec!["user/lib1".to_string(), "user/lib2".to_string()]);
  assert_eq!(
    resolve_dep_repos_for_mono(&args, None),
    vec!["user/lib1", "user/lib2"]
  );
}

/* =====     GENERATE_MONO_CONFIG     ===== */
#[test]
fn test_generate_mono_config_meson() {
  with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join("repos");
    create_dir_all(&repos_path).unwrap();

    let repo_dir = repos_path.join("user-lib1");
    create_dir_all(&repo_dir).unwrap();
    write(repo_dir.join("meson.build"), "project('user-lib1', 'cpp')").unwrap();

    let result = generate_mono_config(
      BuildSystem::Meson,
      tmp_path,
      &repos_path,
      from_ref(&repo_dir),
      &["user/lib1".to_string()],
      ctx,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    let meson_build = tmp_path.join("meson.build");
    assert!(meson_build.exists());

    let content = read_to_string(&meson_build).unwrap();
    assert!(content.contains("user_lib1") || content.contains("user-lib1"));
  });
}

#[test]
fn test_generate_mono_config_npm() {
  with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join("repos");
    create_dir_all(&repos_path).unwrap();

    let result = generate_mono_config(
      BuildSystem::Npm,
      tmp_path,
      &repos_path,
      &[],
      &["user/lib1".to_string(), "user/lib2".to_string()],
      ctx,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    let pkg = tmp_path.join("package.json");
    assert!(pkg.exists());
    let content = read_to_string(&pkg).unwrap();
    assert!(content.contains("workspaces"));
    assert!(content.contains("repos/user-lib1"));
  });
}
