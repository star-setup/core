#![allow(dead_code)]

use star_setup::{
  cli::{
    resolve_with_config, Args, BuildFlags, ConfigFlags, ConnectionFlags, DiagnosticFlags,
    MonoRepoFlags, ProfileFlags,
  },
  config::SetupConfig,
};

pub fn default_args() -> Args {
  Args {
    repo: None,
    yes: false,
    diagnostic: DiagnosticFlags {
      timing: false,
      dry_run: false,
    },
    connection: ConnectionFlags {
      ssh: false,
      https: false,
      verbose: false,
      no_verbose: false,
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
