use star_setup::utils::prerequisites::check_prerequisites;

#[test]
fn test_check_prerequisites_succeeds_with_tools_present() {
  let result = check_prerequisites(false, &mut Vec::new(), false);
  assert!(result.is_ok());
}

#[test]
fn test_check_prerequisites_verbose_outputs_found() {
  let mut output = Vec::new();
  check_prerequisites(true, &mut output, false).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Found git"));
  assert!(out.contains("Found cmake"));
  assert!(out.contains("Found meson"));
}
