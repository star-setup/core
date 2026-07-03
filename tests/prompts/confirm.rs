use star_setup::prompts::confirm;
use crate::common::with_io_input;

#[test]
fn test_confirm_input_cases() {
  let cases = [
    (b"y\n" as &[u8], true, "y accepts"),
    (b"Y\n", true, "Y accepts"),
    (b" y \n", true, "padded y accepts"),
    (b"n\n", false, "n rejects"),
    (b"yes\n", false, "yes rejects"),
  ];

  for (input, expected, name) in cases {
    let result = with_io_input(input, |io| confirm("prompt", false, io));
    assert_eq!(result.unwrap(), expected, "Failed: {name}");
  }
}

#[test]
fn test_confirm_yes_flag_returns_true() {
  assert!(with_io_input(b"", |io| confirm("prompt", true, io)).unwrap());
}

#[test]
fn test_confirm_errors_on_eof() {
  let result = with_io_input(b"", |io| confirm("prompt", false, io));
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}
