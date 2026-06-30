use star_setup::interactive::interactive_mode;
mod common;
use common::{default_resolved, default_resolved_interactive, with_io_input_output};

fn input_with_suffix(prefix: &[u8]) -> Vec<u8> {
  let mut v = prefix.to_vec();
  v.extend_from_slice(b"\n\n\n\n\nn\n");
  v
}

#[test]
fn test_interactive_mode_single_repo() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\n1");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert_eq!(args.repo, Some("user/repo".to_string()));
  assert!(!args.connection.ssh);
  assert!(!args.mono.mono_repo);
}

#[test]
fn test_interactive_mode_ssh_enabled() {
  let input = input_with_suffix(b"user/repo\ny\nn\nn\nn\n1");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved_interactive();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(args.connection.ssh);
}

#[test]
fn test_interactive_mode_mono_repo_with_profile() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\n2\n1\nmyprofile");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(args.mono.mono_repo);
  assert_eq!(args.mono.profile, Some("myprofile".to_string()));
}

#[test]
fn test_interactive_mode_mono_repo_with_manual_repos() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\n2\n2\nuser/lib1 user/lib2");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(args.mono.mono_repo);
  assert_eq!(
    args.mono.repos,
    Some(vec!["user/lib1".to_string(), "user/lib2".to_string()])
  );
}

#[test]
fn test_interactive_mode_skips_repo_prompt_when_set() {
  let input = input_with_suffix(b"n\nn\nn\nn\n1");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    args.repo = Some("already/set".to_string());
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert_eq!(args.repo, Some("already/set".to_string()));
}

#[test]
fn test_interactive_mode_output_contains_header() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\n1");
  let (_, out_str) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(out_str.contains("Star Setup Interactive Mode"));
  assert!(out_str.contains("Interactive mode complete"));
}

#[test]
fn test_interactive_mode_yes_word_not_accepted_for_ssh() {
  let input = input_with_suffix(b"user/repo\nyes\nn\nn\nn\n1");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(!args.connection.ssh);
}

#[test]
fn test_interactive_mode_invalid_mode_then_valid() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\nfoo\n1");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(!args.mono.mono_repo);
}

#[test]
fn test_interactive_mode_invalid_mono_choice_then_valid() {
  let input = input_with_suffix(b"user/repo\nn\nn\nn\nn\n2\nfoo\n1\nmyprofile");
  let (args, _) = with_io_input_output(&input, |io| {
    let mut args = default_resolved();
    interactive_mode(&mut args, io).unwrap();
    args
  });

  assert!(args.mono.mono_repo);
  assert_eq!(args.mono.profile, Some("myprofile".to_string()));
}

#[test]
fn test_interactive_mode_errors_on_eof() {
  let (result, _) = with_io_input_output(b"", |io| {
    let mut args = default_resolved();
    args.repo = None;
    interactive_mode(&mut args, io)
  });

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}
