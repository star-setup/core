use crate::ctx::{IoCtx, RunFlags};
use std::process::Command;

/// Checks if required tools are available on PATH.
/// Returns Result.
/// # Errors
/// Returns an error if any required tool is missing from PATH.
pub fn check_prerequisites(io: &mut IoCtx<'_>, flags: &RunFlags) -> Result<(), String> {
  crate::time!(flags.timing, io.output, "Check prerequisites", {
    let missing: Vec<&str> = ["git", "cmake", "meson"]
      .into_iter()
      .filter(|&tool| {
        let is_missing = Command::new(tool)
          .arg("--version")
          .output()
          .map_or(true, |o| !o.status.success());

        if !is_missing && flags.verbose {
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
