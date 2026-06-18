mod helpers;
use helpers::sink;
use star_setup::utils::{confirm, run_command};

#[test]
fn test_confirm_yes_flag_returns_true() {
  let mut input = b"".as_ref();
  let mut output = sink();
  assert!(confirm("prompt", true, &mut input, &mut output).unwrap());
}

#[test]
fn test_confirm_no_input_returns_false() {
  let input = b"n\n";
  let mut output = sink();
  assert!(!confirm("prompt", false, &mut input.as_ref(), &mut output).unwrap());
}

#[test]
fn test_confirm_yes_input_returns_true() {
  let input = b"y\n";
  let mut output = sink();
  assert!(confirm("prompt", false, &mut input.as_ref(), &mut output).unwrap());
}

#[test]
fn test_run_command_errors_on_empty() {
  let mut output = sink();
  assert!(run_command(&[], None, false, &mut output).is_err());
}

#[test]
fn test_confirm_uppercase_y_returns_true() {
  let input = b"Y\n";
  let mut output = sink();
  assert!(confirm("prompt", false, &mut input.as_ref(), &mut output).unwrap());
}

#[test]
fn test_confirm_yes_word_returns_false() {
  let input = b"yes\n";
  let mut output = sink();
  assert!(!confirm("prompt", false, &mut input.as_ref(), &mut output).unwrap());
}

#[test]
fn test_confirm_padded_y_returns_true() {
  let input = b" y \n";
  let mut output = sink();
  assert!(confirm("prompt", false, &mut input.as_ref(), &mut output).unwrap());
}

#[test]
fn test_confirm_errors_on_eof() {
  let mut input = b"".as_ref();
  let mut output = sink();
  let result = star_setup::utils::confirm("prompt", false, &mut input, &mut output);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("unexpected end of input"));
}
