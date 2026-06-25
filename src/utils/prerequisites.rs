use std::process::Command;
use crate::ctx::IoCtx;

/// Checks if required tools are available on PATH.
/// Returns Result.
/// # Errors
/// Returns an error if any required tool is missing from PATH.
pub fn check_prerequisites(io: &mut IoCtx<'_>) -> Result<(), String> {
  crate::time!(io.timing, io.output, "Check prerequisites", {
    let mut missing: Vec<&str> = Vec::new();

    for tool in &["git", "cmake", "meson"] {
      if Command::new(tool)
        .arg("--version")
        .output()
        .map_or(true, |o| !o.status.success())
      {
        missing.push(tool);
      } else if io.verbose {
        writeln!(io.output, "  Found {tool}").ok();
      }
    }
    if !missing.is_empty() {
      return Err(format!("Missing required tools: {}", missing.join(", ")));
    }
    Ok(())
  })
}
