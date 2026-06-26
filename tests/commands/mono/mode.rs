use super::super::common::{empty_input, make_io, sink, MockRunner};
use star_setup::{
  cli::{
    resolve_with_config, Args, BuildFlags, ConfigFlags, ConnectionFlags, DiagnosticFlags,
    MonoRepoFlags, ProfileFlags,
  },
  commands::mono_repo_mode,
  config::SetupConfig,
  ctx::RunCtx,
};
use tempfile::TempDir;

fn default_resolved_mono(repos: Vec<String>) -> star_setup::cli::ResolvedArgs {
  let args = Args {
    repo: Some("user/test-repo".to_string()),
    yes: true,
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
      mono_repo: true,
      mono_dir: None,
      repos: Some(repos),
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
