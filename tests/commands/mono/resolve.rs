use star_setup::cli::{
  resolve_with_config, Args, BuildFlags, ConfigFlags, ConnectionFlags, MonoRepoFlags, ProfileFlags,
};
use star_setup::commands::{resolve_repos_for_mono, resolve_test_repo};
use star_setup::config::SetupConfig;
#[path = "../../common/mod.rs"]
mod common;
use common::sink;

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

fn default_resolved() -> star_setup::cli::ResolvedArgs {
  let args = Args {
    repo: Some("user/repo".to_string()),
    yes: false,
    connection: ConnectionFlags {
      ssh: false,
      https: false,
      verbose: false,
      no_verbose: false,
    },
    build: BuildFlags {
      build_type: None,
      build_dir: None,
      no_build: false,
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
fn test_resolve_repos_for_mono_empty_profile_errors() {
  let mut config = SetupConfig::new();
  config.profiles.insert("emptyprofile".to_string(), vec![]);
  let mut args = default_resolved();
  args.mono.profile = Some("emptyprofile".to_string());
  let result = resolve_repos_for_mono(&args, &config, "user/repo", &mut sink());
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("has no repositories"));
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
  let result = resolve_repos_for_mono(&args, &config, "user/repo", &mut sink());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), vec!["user/lib1", "user/lib2"]);
}

#[test]
fn test_resolve_repos_for_mono_with_explicit_repos() {
  let config = SetupConfig::new();
  let mut args = default_resolved();
  args.mono.repos = Some(vec!["user/lib1".to_string(), "user/lib2".to_string()]);
  let result = resolve_repos_for_mono(&args, &config, "user/repo", &mut sink());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), vec!["user/lib1", "user/lib2"]);
}

#[test]
fn test_resolve_repos_for_mono_no_repos_or_profile_errors() {
  let config = SetupConfig::new();
  let args = default_resolved();
  let result = resolve_repos_for_mono(&args, &config, "user/repo", &mut sink());
  assert!(result.is_err());
  assert!(result
    .unwrap_err()
    .contains("No repos or profile specified"));
}

#[test]
fn test_resolve_repos_for_mono_profile_not_found_errors() {
  let config = SetupConfig::new();
  let mut args = default_resolved();
  args.mono.profile = Some("nonexistent".to_string());
  let result = resolve_repos_for_mono(&args, &config, "user/repo", &mut sink());
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("not found"));
}
