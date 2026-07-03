use crate::{ctx::IoCtx, prompts::read_input_line};

/// Returns `true` if `yes` is set or the user enters `y`/`Y`.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn confirm(prompt: &str, yes: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  if yes {
    return Ok(true);
  }
  let input = read_input_line(&format!("{prompt} (y/n): "), io)?;
  Ok(input.eq_ignore_ascii_case("y"))
}

/// Prompts the user to confirm or abort an option.
/// # Errors
/// Returns an error if stdin reaches EOF unexpecedly.
pub fn confirm_abort(warning_msg: &str, yes: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  if !confirm(warning_msg, yes, io)? {
    writeln!(io.output, "  Aborted.").ok();
    return Ok(false);
  }
  Ok(true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchConfirm {
  Yes,
  No,
  YesAll,
  NoAll,
}

/// Prompts y/n, plus (when `allow_all`) shortcuts to apply the answer to all remaining items.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn confirm_batch(
  prompt: &str,
  allow_all: bool,
  io: &mut IoCtx<'_>,
) -> Result<BatchConfirm, String> {
  let suffix = if allow_all { " (y/n/a/s)" } else { " (y/n)" };
  loop {
    let input = read_input_line(&format!("{prompt}{suffix}: "), io)?;
    match input.to_lowercase().as_str() {
      "y" => return Ok(BatchConfirm::Yes),
      "n" | "" => return Ok(BatchConfirm::No),
      "a" if allow_all => return Ok(BatchConfirm::YesAll),
      "s" if allow_all => return Ok(BatchConfirm::NoAll),
      _ => {}
    }
  }
}
