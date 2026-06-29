use crate::common::{default_resolved, with_ctx, with_io, MockRunner};
use star_setup::{
  commands::{mono::generate_mono_config, resolve_repos_for_mono, resolve_test_repo},
  config::SetupConfig,
};

// resolve_test_repo tests
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

#[test]
fn test_resolve_repos_for_mono_empty_profile_errors() {
  let mut config = SetupConfig::new();
  config.profiles.insert("emptyprofile".to_string(), vec![]);
  let mut args = default_resolved();
  args.mono.profile = Some("emptyprofile".to_string());

  with_io(|io| {
    let result = resolve_repos_for_mono(&args, &config, "user/repo", io);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("has no repositories"));
  });
}

#[test]
fn test_resolve_repos_for_mono_with_profile() {
  let mut config = SetupConfig::new();
  config.profiles.insert(
    "myprofile".to_string(),
    vec!["user/lib1".to_string(), "user/lib2".to_string()],
  );
  let mut args = default_resolved();
  args.mono.profile = Some("myprofile".to_string());

  with_io(|io| {
    let result = resolve_repos_for_mono(&args, &config, "user/repo", io);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["user/lib1", "user/lib2"]);
  });
}

#[test]
fn test_resolve_repos_for_mono_with_explicit_repos() {
  let config = SetupConfig::new();
  let mut args = default_resolved();
  args.mono.repos = Some(vec!["user/lib1".to_string(), "user/lib2".to_string()]);

  with_io(|io| {
    let result = resolve_repos_for_mono(&args, &config, "user/repo", io);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["user/lib1", "user/lib2"]);
  });
}

#[test]
fn test_resolve_repos_for_mono_no_repos_or_profile_errors() {
  let config = SetupConfig::new();
  let args = default_resolved();

  with_io(|io| {
    let result = resolve_repos_for_mono(&args, &config, "user/repo", io);
    assert!(result.is_err());
    assert!(result
      .unwrap_err()
      .contains("No repos or profile specified"));
  });
}

#[test]
fn test_resolve_repos_for_mono_profile_not_found_errors() {
  let config = SetupConfig::new();
  let mut args = default_resolved();
  args.mono.profile = Some("nonexistent".to_string());

  with_io(|io| {
    let result = resolve_repos_for_mono(&args, &config, "user/repo", io);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
  });
}

#[test]
fn test_generate_mono_config_meson() {
  with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join("repos");
    std::fs::create_dir_all(&repos_path).unwrap();

    let repo_dir = repos_path.join("user-lib1");
    std::fs::create_dir_all(&repo_dir).unwrap();
    std::fs::write(repo_dir.join("meson.build"), "project('user-lib1', 'cpp')").unwrap();

    let result = generate_mono_config(
      star_setup::cli::BuildSystem::Meson,
      tmp_path,
      &repos_path,
      std::slice::from_ref(&repo_dir),
      &["user/lib1".to_string()],
      ctx,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    let meson_build = tmp_path.join("meson.build");
    assert!(meson_build.exists());

    let content = std::fs::read_to_string(&meson_build).unwrap();
    assert!(content.contains("user_lib1") || content.contains("user-lib1"));
  });
}

#[test]
fn test_generate_mono_config_npm() {
  with_ctx(MockRunner::new(), |tmp_path, ctx| {
    let repos_path = tmp_path.join("repos");
    std::fs::create_dir_all(&repos_path).unwrap();

    let result = generate_mono_config(
      star_setup::cli::BuildSystem::Npm,
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
    let content = std::fs::read_to_string(&pkg).unwrap();
    assert!(content.contains("workspaces"));
    assert!(content.contains("repos/user-lib1"));
  });
}
