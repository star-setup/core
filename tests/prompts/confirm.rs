use crate::common::{with_io_input, with_io_input_output};
use star_setup::prompts::{confirm, confirm_batch, BatchConfirm};

/* =====     CONFIRM     ===== */
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

/* =====     CONFIRM_BATCH     ===== */
#[test]
fn test_confirm_batch_input_cases() {
  let cases = [
    (b"y\n" as &[u8], true, BatchConfirm::Yes, "y accepts"),
    (b"Y\n", true, BatchConfirm::Yes, "Y accepts"),
    (b"n\n", true, BatchConfirm::No, "n rejects"),
    (b"\n", true, BatchConfirm::No, "empty defaults to no"),
    (b"a\n", true, BatchConfirm::YesAll, "a accepts all"),
    (b"s\n", true, BatchConfirm::NoAll, "s skips all"),
    (
      b"x\ny\n",
      true,
      BatchConfirm::Yes,
      "invalid input reprompts",
    ),
    (
      b"a\ny\n",
      false,
      BatchConfirm::Yes,
      "a invalid without allow_all",
    ),
    (
      b"s\nn\n",
      false,
      BatchConfirm::No,
      "s invalid without allow_all",
    ),
  ];
  for (input, allow_all, expected, name) in cases {
    let result = with_io_input(input, |io| confirm_batch("prompt", allow_all, io));
    assert_eq!(result.unwrap(), expected, "Failed: {name}");
  }
}

#[test]
fn test_confirm_batch_errors_on_eof() {
  assert!(with_io_input(b"", |io| confirm_batch("prompt", true, io)).is_err());
}

#[test]
fn test_confirm_batch_prompt_suffix() {
  let (_, out) = with_io_input_output(b"y\n", |io| confirm_batch("prompt", true, io));
  assert!(out.contains("(y/n/a/s)"));
  let (_, out) = with_io_input_output(b"y\n", |io| confirm_batch("prompt", false, io));
  assert!(out.contains("(y/n)") && !out.contains("(y/n/a/s)"));
}
