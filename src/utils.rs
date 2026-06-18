//! Utility functions.

use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

/// Returns `true` if `yes` is set or the user enters `y`/`Y`
pub fn confirm(prompt: &str, yes: bool, input: &mut impl BufRead, output: &mut impl Write) -> bool {
  if yes {
    return true;
  }

  write!(output, "{prompt} (y/n): ").ok();
  output.flush().ok();
  let mut line = String::new();
  if input.read_line(&mut line).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  line.trim().eq_ignore_ascii_case("y")
}

/// Checks if required tools are available on PATH.
/// Returns Result.
/// # Errors
/// Returns an error if any required tool is missing from PATH.
pub fn check_prerequisites(verbose: bool, output: &mut impl Write) -> Result<(), String> {
  let mut missing: Vec<&str> = Vec::new();
  for tool in &["git", "cmake"] {
    if Command::new(tool)
      .arg("--version")
      .output()
      .map_or(true, |o| !o.status.success())
    {
      missing.push(tool);
    } else if verbose {
      writeln!(output, "Found {tool}").ok();
    }
  }
  if !missing.is_empty() {
    return Err(format!("Missing required tools: {}", missing.join(", ")));
  }
  Ok(())
}

/// Runs a shell command with optional working directory.
/// Returns Result.
/// # Errors
/// Returns an error if the command is empty, fails to spawn, or exits with a non-zero status.
pub fn run_command(
  cmd: &[&str],
  cwd: Option<&Path>,
  verbose: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  if cmd.is_empty() {
    return Err("No command provided".to_string());
  }

  if verbose {
    writeln!(output, "Running: {}", cmd.join(" ")).ok();
    if let Some(dir) = cwd {
      writeln!(output, "  in directory: {}", dir.display()).ok();
    }
  }

  let mut command = Command::new(cmd[0]);
  command.stdin(Stdio::null());
  if cmd[0] == "git" {
    command.env("GIT_TERMINAL_PROMPT", "0");
    if std::env::var("GIT_SSH_COMMAND").is_err() {
      command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
  }

  if verbose {
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
  } else {
    command.stdout(Stdio::null());
    command.stderr(Stdio::piped());
  }

  command.args(&cmd[1..]);
  if let Some(dir) = cwd {
    command.current_dir(dir);
  }

  if verbose {
    let status = command
      .status()
      .map_err(|e| format!("Failed to run command: {e}"))?;
    if status.success() {
      return Ok(());
    }
    return Err(format!("Command failed with exit code: {status}"));
  }

  let mut child = command
    .spawn()
    .map_err(|e| format!("Failed to start command: {e}"))?;
  let stderr_handle = child.stderr.take();
  let stderr_thread = thread::spawn(move || {
    let mut s = String::new();
    if let Some(mut h) = stderr_handle {
      h.read_to_string(&mut s).ok();
    }
    s
  });

  let status = child
    .wait()
    .map_err(|e| format!("Failed to wait for command: {e}"))?;
  let stderr = stderr_thread.join().unwrap_or_default();
  if status.success() {
    Ok(())
  } else {
    let msg: &str = stderr.trim();
    if msg.is_empty() {
      Err(format!("Command failed with exit code: {status}"))
    } else {
      Err(format!("Command failed with exit code: {status}\n{msg}"))
    }
  }
}
