use star_setup::prompts::confirm;
use star_setup::prompts::{ask, ask_default, ask_yesno};
mod common;
use common::sink;

#[test]
fn test_ask_errors_on_eof() {
  let result = ask("prompt", &mut b"".as_ref(), &mut Vec::new());
  assert!(result.is_err());
}

#[test]
fn test_ask_default_errors_on_eof() {
  let result = ask_default("prompt", "default", &mut b"".as_ref(), &mut Vec::new());
  assert!(result.is_err());
}

#[test]
fn test_ask_yesno_errors_on_eof() {
  let result = ask_yesno("prompt", true, &mut b"".as_ref(), &mut Vec::new());
  assert!(result.is_err());
}

#[test]
fn test_ask_default_returns_input_when_not_empty() {
  let result = ask_default(
    "prompt",
    "default",
    &mut b"custom\n".as_ref(),
    &mut Vec::new(),
  )
  .unwrap();
  assert_eq!(result, "custom");
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
