use super::common::with_io_input_output;
use star_setup::commands::{print_mode_header, ModeHeader};

#[test]
fn test_print_mode_header_repo_name_without_test_repo() {
  let ((), out) = with_io_input_output(b"", |io| {
    print_mode_header(
      &ModeHeader {
        mode: "Single Repository Mode",
        test_repo: None,
        repo_name: Some("myrepo"),
        use_ssh: false,
        mono_dir: None,
        profile: None,
        lib_count: None,
      },
      io,
    );
  });

  assert!(out.contains("Repository: myrepo"));
}
