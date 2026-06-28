use star_setup::{
  ctx::IoCtx,
  prompts::{ask, ask_default, ask_yesno, confirm},
};
mod common;
use common::make_io;

fn run_prompt_test<T, F>(input: &[u8], test_logic: F) -> T
where
  F: FnOnce(&mut IoCtx<'_>) -> T,
{
  let mut input_slice = input;
  let mut output = Vec::new();
  let mut io = make_io(&mut input_slice, &mut output);
  test_logic(&mut io)
}

#[test]
fn test_ask_errors_on_eof() {
  assert!(run_prompt_test(b"", |io| ask("prompt", io)).is_err());
}

#[test]
fn test_ask_default_errors_on_eof() {
  assert!(run_prompt_test(b"", |io| ask_default("prompt", "default", io)).is_err());
}

#[test]
fn test_ask_yesno_errors_on_eof() {
  assert!(run_prompt_test(b"", |io| ask_yesno("prompt", true, io)).is_err());
}

#[test]
fn test_ask_default_returns_input_when_not_empty() {
  let result = run_prompt_test(b"custom\n", |io| ask_default("prompt", "default", io));
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
    let result = run_prompt_test(input, |io| confirm("prompt", false, io));
    assert_eq!(result.unwrap(), expected, "Failed: {name}");
  }
}

#[test]
fn test_confirm_yes_flag_returns_true() {
  assert!(run_prompt_test(b"", |io| confirm("prompt", true, io)).unwrap());
}

#[test]
fn test_confirm_errors_on_eof() {
  let result = run_prompt_test(b"", |io| confirm("prompt", false, io));
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}
