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
  assert_eq!(String::from_utf8(output).unwrap(), "Would run: git clone foo\n");
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
  runner.run(&["cmake", ".."], Some(Path::new("/tmp/build")), &mut io).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Would run: cmake .."));
  assert!(out.contains("  in directory: /tmp/build"));
}
