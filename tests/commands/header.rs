#[test]
fn test_print_mode_header_repo_name_without_test_repo() {
  use star_setup::commands::header::{print_mode_header, ModeHeader};
  let mut output = Vec::new();
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
    &mut output,
  );
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Repository: myrepo"));
}
