use crate::ctx::IoCtx;
use std::process::Command;

/// Checks if required tools are available on PATH.
/// Returns Result.
/// # Errors
/// Returns an error if any required tool is missing from PATH.
pub fn check_prerequisites(io: &mut IoCtx<'_>) -> Result<(), String> {
  crate::time!(io.timing, io.output, "Check prerequisites", {
    let missing: Vec<&str> = ["git", "cmake", "meson"]
      .into_iter()
      .filter(|&tool| {
        let is_missing = Command::new(tool)
          .arg("--version")
          .output()
          .map_or(true, |o| !o.status.success());

        if !is_missing && io.verbose {
          let _ = writeln!(io.output, "   Found {tool}");
        }
        is_missing
      })
      .collect();

    if !missing.is_empty() {
      return Err(format!("Missing required tools: {}", missing.join(", ")));
    }

    Ok(())
  })
}
