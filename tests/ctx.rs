use star_setup::ctx::{DryRunRunner, IoCtx, ProcessRunner, Runner};
use std::path::Path;
mod common;
use common::make_io;

fn run_runner_test<R, F>(dry_run: bool, mut runner: R, test_logic: F) -> String
where
  R: Runner,
  F: FnOnce(&mut R, &mut IoCtx<'_>),
{
  let mut input = b"".as_ref();
  let mut output = Vec::new();

  let mut io = make_io(&mut input, &mut output);
  io.dry_run = dry_run;

  test_logic(&mut runner, &mut io);
  String::from_utf8(output).unwrap_or_default()
}

#[test]
fn test_process_runner_runs_command() {
  run_runner_test(false, ProcessRunner, |runner, io| {
    assert!(runner.run(&["git", "--version"], None, io).is_ok());
  });
}

#[test]
fn test_dry_run_runner_prints_command() {
  let output = run_runner_test(true, DryRunRunner, |runner, io| {
    runner.run(&["git", "clone", "foo"], None, io).unwrap();
  });
  assert_eq!(output, "Would run: git clone foo\n");
}

#[test]
fn test_dry_run_runner_prints_cwd() {
  let output = run_runner_test(true, DryRunRunner, |runner, io| {
    runner
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
