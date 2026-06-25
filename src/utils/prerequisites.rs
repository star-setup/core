use std::io::Write;
use std::process::Command;

/// Checks if required tools are available on PATH.
/// Returns Result.
/// # Errors
/// Returns an error if any required tool is missing from PATH.
pub fn check_prerequisites(
  verbose: bool,
  output: &mut impl Write,
  timing: bool,
) -> Result<(), String> {
  crate::time!(timing, output, "Check prerequisites", {
    let mut missing: Vec<&str> = Vec::new();

    for tool in &["git", "cmake", "meson"] {
      if Command::new(tool)
        .arg("--version")
        .output()
        .map_or(true, |o| !o.status.success())
      {
        missing.push(tool);
      } else if verbose {
        writeln!(output, "  Found {tool}").ok();
      }
    }
    if !missing.is_empty() {
      return Err(format!("Missing required tools: {}", missing.join(", ")));
    }
    Ok(())
  })
}
