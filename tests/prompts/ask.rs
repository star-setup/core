use crate::common::with_io_input;
use star_setup::prompts::{ask, ask_default, ask_yesno};

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
