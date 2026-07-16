use crate::common::with_io_input_output;
use star_setup::commands::{print_mode_header, ModeHeader};

#[test]
fn test_print_mode_header_repo_name_without_test_repo() {
  let ((), out) = with_io_input_output(b"", |io| {
    print_mode_header(
      &ModeHeader {
        mode: "Single Repository Mode",
        test_repos: &[],
        repo_name: Some("myrepo"),
        use_ssh: false,
        mono_dir: None,
        profile: None,
        lib_count: None,
        repo_count: None,
        verbose: false,
      },
      io,
    );
  });

  assert!(out.contains("Repository: myrepo"));
}

#[test]
fn test_print_mode_header_test_repos_count_when_not_verbose() {
  let ((), out) = with_io_input_output(b"", |io| {
    print_mode_header(
      &ModeHeader {
        mode: "Profile",
        test_repos: &["user/game1".to_string(), "user/game2".to_string()],
        repo_name: None,
        use_ssh: false,
        mono_dir: None,
        profile: None,
        lib_count: None,
        repo_count: None,
        verbose: false,
      },
      io,
    );
  });
  assert!(out.contains("Test Repositories: 2"));
  assert!(!out.contains("user/game1"));
}

#[test]
fn test_print_mode_header_test_repos_lists_when_verbose() {
  let ((), out) = with_io_input_output(b"", |io| {
    print_mode_header(
      &ModeHeader {
        mode: "Profile",
        test_repos: &["user/game1".to_string()],
        repo_name: None,
        use_ssh: false,
        mono_dir: None,
        profile: None,
        lib_count: None,
        repo_count: None,
        verbose: true,
      },
      io,
    );
  });
  assert!(out.contains("user/game1"));
}
