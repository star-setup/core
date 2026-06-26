use crate::utils::process::run_command;
use std::{
  io::{BufRead, Write},
  path::Path,
};

/// IO context passed to functions that need input/output and behavioral flags.
pub struct IoCtx<'a> {
  pub input: &'a mut dyn BufRead,
  pub output: &'a mut dyn Write,
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
  fn run(&mut self, cmd: &[&str], cwd: Option<&Path>, io: &mut IoCtx<'_>) -> Result<(), String>;
}

/// Full execution context combining IO and a command runner.
pub struct RunCtx<'a> {
  pub io: IoCtx<'a>,
  pub runner: &'a mut dyn Runner,
}

/// Runner that executes commands.
pub struct ProcessRunner;
impl Runner for ProcessRunner {
  fn run(&mut self, cmd: &[&str], cwd: Option<&Path>, io: &mut IoCtx<'_>) -> Result<(), String> {
    run_command(cmd, cwd, io.verbose, io.output)
  }
}

/// Runner that prints commands instead of executing them.
pub struct DryRunRunner;
impl Runner for DryRunRunner {
  fn run(&mut self, cmd: &[&str], cwd: Option<&Path>, io: &mut IoCtx<'_>) -> Result<(), String> {
    writeln!(io.output, "Would run: {}", cmd.join(" ")).map_err(|e| e.to_string())?;
    if let Some(dir) = cwd {
      writeln!(io.output, "  in directory: {}", dir.display()).map_err(|e| e.to_string())?;
    }
    Ok(())
  }
}
