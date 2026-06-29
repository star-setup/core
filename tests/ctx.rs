use star_setup::ctx::{DryRunRunner, ProcessRunner, Runner};
use std::path::Path;
mod common;
use common::{with_io, with_io_output};

#[test]
fn test_process_runner_runs_command() {
  with_io(|io| {
    assert!(ProcessRunner.run(&["git", "--version"], None, io).is_ok());
  });
}

#[test]
fn test_dry_run_runner_prints_command() {
  let (_, output) = with_io_output(|io| {
    io.dry_run = true;
    DryRunRunner
      .run(&["git", "clone", "foo"], None, io)
      .unwrap();
  });
  assert_eq!(output, "Would run: git clone foo\n");
}

#[test]
fn test_dry_run_runner_prints_cwd() {
  let (_, output) = with_io_output(|io| {
    io.dry_run = true;
    DryRunRunner
      .run(&["cmake", ".."], Some(Path::new("/tmp/build")), io)
      .unwrap();
  });
  assert!(output.contains("Would run: cmake .."));
  assert!(output.contains("  in directory: /tmp/build"));
}

#[test]
fn test_process_runner_captures_output() {
  let mut runner = ProcessRunner;
  let result = runner.run_capture(&["git", "--version"], None);
  assert!(result.is_ok());
  assert!(result.unwrap().contains("git version"));
}

#[test]
fn test_dry_run_runner_capture_returns_empty() {
  let mut runner = DryRunRunner;
  let result = runner.run_capture(&["git", "--version"], None);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), "");
}

#[test]
fn test_process_runner_capture_errors_on_empty() {
  let mut runner = ProcessRunner;
  assert!(runner.run_capture(&[], None).is_err());
}

#[test]
fn test_process_runner_capture_with_cwd() {
  let mut runner = ProcessRunner;
  let result = runner.run_capture(&["git", "--version"], Some(Path::new(".")));
  assert!(result.is_ok());
}

#[test]
fn test_process_runner_capture_errors_on_failure() {
  let mut runner = ProcessRunner;
  let result = runner.run_capture(&["git", "invalid-command-xyz"], None);
  assert!(result.is_err());
}
