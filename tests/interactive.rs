use star_setup::cli::{
  resolve_with_config, Args, BuildFlags, ConfigFlags, ConnectionFlags, MonoRepoFlags, ProfileFlags,
};
use star_setup::config::types::SetupConfig;
use star_setup::interactive::interactive_mode;

fn default_resolved() -> star_setup::cli::ResolvedArgs {
  let args = Args {
    repo: None,
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

fn input_with_suffix(prefix: &[u8]) -> Vec<u8> {
  let mut v = prefix.to_vec();
  v.extend_from_slice(b"\n\n\n\n\nn\n");
  v
}

#[test]
fn test_interactive_mode_single_repo() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert_eq!(args.repo, Some("user/repo".to_string()));
  assert!(!args.connection.ssh);
  assert!(!args.mono.mono_repo);
}

#[test]
fn test_interactive_mode_ssh_enabled() {
  let input = input_with_suffix(b"user/repo\ny\nn\nn\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(args.connection.ssh);
}

#[test]
fn test_interactive_mode_mono_repo_with_profile() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\n2\n1\nmyprofile");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(args.mono.mono_repo);
  assert_eq!(args.mono.profile, Some("myprofile".to_string()));
}

#[test]
fn test_interactive_mode_mono_repo_with_manual_repos() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\n2\n2\nuser/lib1 user/lib2");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(args.mono.mono_repo);
  assert_eq!(
    args.mono.repos,
    Some(vec!["user/lib1".to_string(), "user/lib2".to_string()])
  );
}

#[test]
fn test_interactive_mode_skips_repo_prompt_when_set() {
  let input = input_with_suffix(b"n\nn\nn\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  args.repo = Some("already/set".to_string());
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert_eq!(args.repo, Some("already/set".to_string()));
}

#[test]
fn test_interactive_mode_output_contains_header() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  let out_str = String::from_utf8(output).unwrap();
  assert!(out_str.contains("Star Setup Interactive Mode"));
  assert!(out_str.contains("Interactive mode complete"));
}

#[test]
fn test_interactive_mode_errors_on_eof() {
  let input = b"";
  let mut output = Vec::new();
  let mut args = default_resolved();
  args.repo = None;
  let result = interactive_mode(&mut args, &mut input.as_ref(), &mut output);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}

#[test]
fn test_interactive_mode_yes_word_not_accepted_for_ssh() {
  let input = input_with_suffix(b"user/repo\nyes\nn\nn\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(!args.connection.ssh);
}

#[test]
fn test_interactive_mode_invalid_mode_then_valid() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nfoo\n1");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(!args.mono.mono_repo);
}

#[test]
fn test_interactive_mode_invalid_mono_choice_then_valid() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\n2\nfoo\n1\nmyprofile");
  let mut output = Vec::new();
  let mut args = default_resolved();
  interactive_mode(&mut args, &mut input.as_ref(), &mut output).unwrap();
  assert!(args.mono.mono_repo);
  assert_eq!(args.mono.profile, Some("myprofile".to_string()));
}
