use star_setup::prompts::{ask, ask_default, ask_yesno, confirm};
mod common;
use common::make_io;

#[test]
fn test_ask_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert!(ask("prompt", &mut io).is_err());
}

#[test]
fn test_ask_default_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert!(ask_default("prompt", "default", &mut io).is_err());
}

#[test]
fn test_ask_yesno_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert!(ask_yesno("prompt", true, &mut io).is_err());
}

#[test]
fn test_ask_default_returns_input_when_not_empty() {
  let mut input = b"custom\n".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert_eq!(ask_default("prompt", "default", &mut io).unwrap(), "custom");
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
    let mut input = input;
    let mut output = Vec::new();
    let mut io = make_io(&mut input, &mut output);
    assert_eq!(
      confirm("prompt", false, &mut io).unwrap(),
      expected,
      "Failed: {name}"
    );
  }
}

#[test]
fn test_confirm_yes_flag_returns_true() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert!(confirm("prompt", true, &mut io).unwrap());
}

#[test]
fn test_confirm_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  let result = confirm("prompt", false, &mut io);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}
