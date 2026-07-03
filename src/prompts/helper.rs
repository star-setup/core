use crate::ctx::IoCtx;

/// Internal helper to print a prompt, flush, and read a trimmed line of input.
/// # Errors
/// Returns an error if an unexpected EOF is encountered.
pub fn read_input_line(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  write!(io.output, "{prompt}").ok();
  io.output.flush().ok();

  let mut line = String::new();
  if io.input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  Ok(line.trim().to_string())
}
