use star_setup::prompts::{ask, ask_default, ask_yesno, confirm};
mod common;
use common::with_io_input;

#[test]
fn test_ask_errors_on_eof() {
  assert!(with_io_input(b"", |io| ask("prompt", io)).is_err());
}

#[test]
fn test_ask_default_errors_on_eof() {
  assert!(with_io_input(b"", |io| ask_default("prompt", "default", io)).is_err());
}

#[test]
fn test_ask_yesno_errors_on_eof() {
  assert!(with_io_input(b"", |io| ask_yesno("prompt", true, io)).is_err());
}

#[test]
fn test_ask_default_returns_input_when_not_empty() {
  let result = with_io_input(b"custom\n", |io| ask_default("prompt", "default", io));
  assert_eq!(result.unwrap(), "custom");
}

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
