use std::io::BufRead;
use std::io::Write;

/// Returns `true` if `yes` is set or the user enters `y`/`Y`.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn confirm(
  prompt: &str,
  yes: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<bool, String> {
  if yes {
    return Ok(true);
  }

  write!(output, "{prompt} (y/n): ").ok();
  output.flush().ok();
  let mut line = String::new();
  if input.read_line(&mut line).unwrap_or(0) == 0 {
    return Err("unexpected end of input".to_string());
  }
  Ok(line.trim().eq_ignore_ascii_case("y"))
}
