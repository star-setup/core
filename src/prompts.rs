//! Interactive prompt helpers.

use crate::ctx::IoCtx;

/// Internal helper to print a prompt, flush, and read a trimmed line of input.
fn read_input_line(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  write!(io.output, "{prompt}").ok();
  io.output.flush().ok();

  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  Ok(line.trim().to_string())
}

/// Prompts the user for a required string value.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  read_input_line(&format!("{prompt}: "), io)
}

/// Prompts the user for a string value, returning `default` if the input is empty.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask_default(prompt: &str, default: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  let input = read_input_line(&format!("{prompt} [{default}]: "), io)?;
  Ok(if input.is_empty() {
    default.to_string()
  } else {
    input
  })
}

/// Prompts the user for a yes/no answer, returning `default` if the input is empty.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask_yesno(prompt: &str, default: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  let default_char = if default { "Y" } else { "N" };
  let input = read_input_line(&format!("{prompt} (y/n) [{default_char}]: "), io)?;
  Ok(if input.is_empty() {
    default
  } else {
    input.eq_ignore_ascii_case("y")
  })
}

/// Prompts the user to select from a numbered list of options.
/// Returns the zero-based index of the selected option.
/// # Errors
/// Returns an error on EOF or if the selection is out of range.
pub fn ask_choice(prompt: &str, options: &[&str], io: &mut IoCtx<'_>) -> Result<usize, String> {
  writeln!(io.output, "{prompt}").ok();
  for (i, opt) in options.iter().enumerate() {
    writeln!(io.output, "  {}) {opt}", i + 1).ok();
  }
  loop {
    let input = read_input_line("Select: ", io)?;
    if let Ok(n) = input.parse::<usize>() {
      if n >= 1 && n <= options.len() {
        return Ok(n - 1);
      }
    }
  }
}

/// Prompts `ask_yesno` only if the condition isn't already met.
/// # Errors
/// Returns an error on EOF or if the selection is out of range.
pub fn ask_bool_if(prompt: &str, current_val: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  if current_val {
    Ok(current_val)
  } else {
    ask_yesno(prompt, false, io)
  }
}

/// Repeatedly ask until a non-empty string is provided.
/// # Errors
/// Returns an error on EOF or if the selection is out of range.
pub fn ask_required(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  loop {
    let response = ask(prompt, io)?;
    if !response.is_empty() {
      return Ok(response);
    }
  }
}

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
    writeln!(io.output, "Aborted.").ok();
    return Ok(false);
  }
  Ok(true)
}
