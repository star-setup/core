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
}

/// Trait for executing shell commands.
/// # Errors
/// Returns an error if the command fails to spawn or exits with a non-zero status.
pub trait Runner {
  /// Executes a shell command with optional working directory.
  /// # Errors
  /// Returns an error if the command fails to spawn or exits with a non-zero status.
  fn run(&mut self, cmd: &[&str], cwd: Option<&Path>, output: &mut dyn Write)
    -> Result<(), String>;
}

/// Full execution context combining IO and a command runner.
pub struct RunCtx<'a> {
  pub io: IoCtx<'a>,
  pub runner: &'a mut dyn Runner,
}

pub struct ProcessRunner {
  pub verbose: bool,
}
impl Runner for ProcessRunner {
  fn run(
    &mut self,
    cmd: &[&str],
    cwd: Option<&Path>,
    output: &mut dyn Write,
  ) -> Result<(), String> {
    run_command(cmd, cwd, self.verbose, output)
  }
}
