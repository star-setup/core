#![allow(dead_code)]

use star_setup::{
  cli::{resolve_with_config, Args, BuildFlags, ConnectionFlags, DiagnosticFlags, MonoRepoFlags},
  config::SetupConfig,
};

pub fn default_args() -> Args {
  Args {
    repo: None,
    yes: false,
    config_name: None,
    command: None,
    diagnostic: DiagnosticFlags {
      verbose: false,
      no_verbose: false,
      timing: false,
      no_timing: false,
      dry_run: false,
      no_dry_run: false,
    },
    connection: ConnectionFlags {
      ssh: false,
      https: false,
    },
    build: BuildFlags {
      build_type: None,
      build_dir: None,
      build_system: None,
      no_build: false,
      build: false,
      clean: false,
      no_clean: false,
      cmake_flags: vec![],
      meson_flags: vec![],
      watch: false,
      no_watch: false,
    },
    mono: MonoRepoFlags {
      mono_repo: false,
      mono_dir: None,
      repos: None,
      profile: None,
    },
  }
}

pub fn default_resolved() -> star_setup::cli::ResolvedArgs {
  let mut args = default_args();
  args.repo = Some("user/repo".to_string());
  args.build.no_build = true;
  resolve_with_config(args, &SetupConfig::new()).unwrap()
}

pub fn default_resolved_with_no_build(no_build: bool) -> star_setup::cli::ResolvedArgs {
  let mut args = default_args();
  args.repo = Some("user/repo".to_string());
  args.build.no_build = no_build;
  resolve_with_config(args, &SetupConfig::new()).unwrap()
}

pub fn default_resolved_interactive() -> star_setup::cli::ResolvedArgs {
  resolve_with_config(default_args(), &SetupConfig::new()).unwrap()
}

pub fn default_resolved_mono(repos: Vec<String>) -> star_setup::cli::ResolvedArgs {
  let mut args = default_args();
  args.repo = Some("user/test-repo".to_string());
  args.yes = true;
  args.build.no_build = true;
  args.mono.mono_repo = true;
  args.mono.repos = Some(repos);
  resolve_with_config(args, &SetupConfig::new()).unwrap()
}
