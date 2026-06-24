use star_setup::prompts::{ask, ask_default, ask_yesno};

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
