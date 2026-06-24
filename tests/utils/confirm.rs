use super::common::sink;
use star_setup::utils::confirm::confirm;
use star_setup::utils::process::run_command;

#[test]
fn test_confirm_input_cases() {
  let cases = [
    (b"y\n" as &[u8], true, "y accepts"),
    (b"Y\n", true, "Y accepts"),
    (b" y \n", true, "padded y accepts"),
    (b"n\n", false, "n rejects"),
    (b"yes\n", false, "yes rejects"),
  ];
  for (mut input, expected, name) in cases {
    let mut output = sink();
    assert_eq!(
      confirm("prompt", false, &mut input, &mut output).unwrap(),
      expected,
      "Failed: {name}"
    );
  }
}

#[test]
fn test_confirm_yes_flag_returns_true() {
  let mut input = b"".as_ref();
  let mut output = sink();
  assert!(confirm("prompt", true, &mut input, &mut output).unwrap());
}

#[test]
fn test_confirm_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = sink();
  let result = confirm("prompt", false, &mut input, &mut output);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}

#[test]
fn test_run_command_errors_on_empty() {
  let mut output = sink();
  assert!(run_command(&[], None, false, &mut output).is_err());
}
