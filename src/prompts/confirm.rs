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
