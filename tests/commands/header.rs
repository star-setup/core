use super::common::{empty_input, make_io, sink};
use star_setup::commands::{print_mode_header, ModeHeader};

#[test]
fn test_print_mode_header_repo_name_without_test_repo() {
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
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
    &mut io,
  );
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Repository: myrepo"));
}
