//! Interactive prompt helpers.

use crate::ctx::IoCtx;

/// Prompts the user for a required string value.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  write!(io.output, "{prompt}: ").ok();
  io.output.flush().ok();
  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  Ok(line.trim().to_string())
}

/// Prompts the user for a string value, returning `default` if the input is empty.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask_default(prompt: &str, default: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  write!(io.output, "{prompt} [{default}]: ").ok();
  io.output.flush().ok();
  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  let val = line.trim().to_string();
  Ok(if val.is_empty() {
    default.to_string()
  } else {
    val
  })
}

/// Prompts the user for a yes/no answer, returning `default` if the input is empty.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn ask_yesno(prompt: &str, default: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  let default_char = if default { "Y" } else { "N" };
  write!(io.output, "{prompt} (y/n) [{default_char}]: ").ok();
  io.output.flush().ok();
  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  let val = line.trim().to_lowercase();
  Ok(if val.is_empty() { default } else { val.eq("y") })
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
    write!(io.output, "Select: ").ok();
    io.output.flush().ok();
    let mut line = String::new();
    if io.input.read_line(&mut line).unwrap_or(0) == 0 {
      return Err("unexpected end of input".to_string());
    }
    let val = line.trim();
    if let Ok(n) = val.parse::<usize>() {
      if n >= 1 && n <= options.len() {
        return Ok(n - 1);
      }
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

  write!(io.output, "{prompt} (y/n): ").ok();
  io.output.flush().ok();
  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  Ok(line.trim().eq_ignore_ascii_case("y"))
}
