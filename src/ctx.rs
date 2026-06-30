use crate::utils::process::run_command;
use std::{
  error::Error,
  io::{BufRead, Write},
  path::Path,
};

/// IO context passed to functions that need input/output and behavioral flags.
pub struct IoCtx<'a> {
  pub input: &'a mut dyn BufRead,
  pub output: &'a mut dyn Write,
}

/// Behavioral execution flags.
#[derive(Copy, Clone)]
pub struct RunFlags {
  pub verbose: bool,
  pub timing: bool,
  pub dry_run: bool,
}

/// Trait for executing shell commands.
/// # Errors
/// Returns an error if the command fails to spawn or exits with a non-zero status.
pub trait Runner {
  /// Executes a shell command with optional working directory.
  /// # Errors
  /// Returns an error if the command fails to spawn or exits with a non-zero status.
  fn run(
    &mut self,
    cmd: &[&str],
    cwd: Option<&Path>,
    flags: RunFlags,
    output: &mut dyn Write,
  ) -> Result<(), String>;

  /// Executes a shell command and captures stdout as a string.
  /// # Errors
  /// Returns an error if the command fails to spawn or exits with a non-zero status.
  fn run_capture(&mut self, cmd: &[&str], cwd: Option<&Path>) -> Result<String, String>;
}

/// Runner that executes commands.
pub struct ProcessRunner;
impl Runner for ProcessRunner {
  fn run(
    &mut self,
    cmd: &[&str],
    cwd: Option<&Path>,
    flags: RunFlags,
    output: &mut dyn Write,
  ) -> Result<(), String> {
    run_command(cmd, cwd, flags.verbose, output)
  }

  fn run_capture(&mut self, cmd: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    if cmd.is_empty() {
      return Err("No command provided".to_string());
    }
    let mut command = std::process::Command::new(cmd[0]);
    command.args(&cmd[1..]);
    if let Some(dir) = cwd {
      command.current_dir(dir);
    }
    let output = command
      .output()
      .map_err(|e| format!("Failed to run command: {e}"))?;
    if output.status.success() {
      Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
      Err(format!(
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
      ))
    }
  }
}

/// Runner that prints commands instead of executing them.
pub struct DryRunRunner;
impl Runner for DryRunRunner {
  fn run(
    &mut self,
    cmd: &[&str],
    cwd: Option<&Path>,
    _flags: RunFlags,
    output: &mut dyn Write,
  ) -> Result<(), String> {
    writeln!(output, "Would run: {}", cmd.join(" ")).map_err(|e| e.to_string())?;
    if let Some(dir) = cwd {
      writeln!(output, "  in directory: {}", dir.display()).map_err(|e| e.to_string())?;
    }
    Ok(())
  }

  fn run_capture(&mut self, _cmd: &[&str], _cwd: Option<&Path>) -> Result<String, String> {
    Ok(String::new())
  }
}

/// Full execution context combining IO and a command runner.
pub struct RunCtx<'io, 'run> {
  pub io: IoCtx<'io>,
  pub flags: RunFlags,
  pub runner: &'run mut dyn Runner,
}

/// Helper to quickly execute a workspace/repo task with the correct runner.
/// # Errors
/// Returns an error if the closure function `f` execution returns an error block.
pub fn with_runner<F>(io: IoCtx, flags: RunFlags, f: F) -> Result<(), Box<dyn Error>>
where
  F: FnOnce(&mut RunCtx) -> Result<(), Box<dyn Error>>,
{
  let mut dry;
  let mut real;

  let runner: &mut dyn Runner = if flags.dry_run {
    dry = DryRunRunner;
    &mut dry
  } else {
    real = ProcessRunner;
    &mut real
  };

  let mut ctx = RunCtx { io, flags, runner };
  f(&mut ctx)
}
