//! Utility functions for ecosystem-setup.

use std::process::Command;
use std::path::Path;

/// Checks if required tools are available on PATH.
/// Returns Result.
pub fn check_prerequisites(verbose: bool) -> Result<(), String> {
  let mut missing: Vec<&str> = Vec::new();
  for tool in &["git", "cmake"] {
    if Command::new(tool).arg("--version").output().is_err() {
      missing.push(tool);
    } else if verbose {
      println!("Found {tool}");
    }
  }
  if !missing.is_empty() {
    return Err(format!("Missing required tools: {}", missing.join(", ")));
  }
  Ok(())
}

/// Runs a shell command with optional working directory.
/// Returns Result.
pub fn run_command(cmd: &[&str], cwd: Option<&Path>, verbose: bool) -> Result<(), String> {
  if cmd.is_empty() { return Err("No command provided".to_string()); }

  if verbose {
    println!("Running: {}", cmd.join(" "));
    if let Some(dir) = cwd { println!("  in directory: {}", dir.display()); }
  }

  let mut command = Command::new(cmd[0]);
  command.args(&cmd[1..]);
  if let Some(dir) = cwd { command.current_dir(dir); }
  if !verbose {
    command.stdout(std::process::Stdio::null())
           .stderr(std::process::Stdio::null());
  }
  match command.status() {
    Ok(status) if status.success() => Ok(()),
    Ok(status) => Err(format!("Command failed with exit code: {status}")),
    Err(e)     => Err(format!("Error running command: {e}")),
  }
}
