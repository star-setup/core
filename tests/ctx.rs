use star_setup::ctx::{DryRunRunner, IoCtx, ProcessRunner, Runner};
use std::path::Path;

#[test]
fn test_process_runner_runs_command() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut runner = ProcessRunner;
  let mut io = IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: false,
    timing: false,
    dry_run: false,
  };
  assert!(runner.run(&["git", "--version"], None, &mut io).is_ok());
}

#[test]
fn test_dry_run_runner_prints_command() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut runner = DryRunRunner;
  let mut io = IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: false,
    timing: false,
    dry_run: true,
  };
  runner.run(&["git", "clone", "foo"], None, &mut io).unwrap();
  assert_eq!(
    String::from_utf8(output).unwrap(),
    "Would run: git clone foo\n"
  );
}

#[test]
fn test_dry_run_runner_prints_cwd() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut runner = DryRunRunner;
  let mut io = IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: false,
    timing: false,
    dry_run: true,
  };
  runner
    .run(&["cmake", ".."], Some(Path::new("/tmp/build")), &mut io)
    .unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Would run: cmake .."));
  assert!(out.contains("  in directory: /tmp/build"));
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
