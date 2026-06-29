use crate::common::with_io_output;
use star_setup::utils::process::run_command;

#[test]
fn test_run_command_errors_on_empty() {
  let (result, _) = with_io_output(|io| run_command(&[], None, false, io.output));
  assert!(result.is_err());
}

#[test]
fn test_run_command_verbose_outputs_command() {
  let ((), out) = with_io_output(|io| {
    run_command(&["git", "--version"], None, true, io.output).unwrap();
  });
  assert!(out.contains("Running: git --version"));
}

#[test]
fn test_run_command_verbose_outputs_cwd() {
  let ((), out) = with_io_output(|io| {
    let tmp = tempfile::TempDir::new().unwrap();
    run_command(&["git", "--version"], Some(tmp.path()), true, io.output).unwrap();
  });
  assert!(out.contains("in directory:"));
}

#[test]
fn test_run_command_fails_with_stderr() {
  let (result, _) =
    with_io_output(|io| run_command(&["git", "clone", "not-a-real-repo"], None, false, io.output));
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Command failed"));
}

#[test]
fn test_run_command_fails_no_stderr() {
  let (result, _) =
    with_io_output(|io| run_command(&["git", "invalid-command-xyz"], None, false, io.output));
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Command failed"));
}

#[test]
fn test_run_command_verbose_fails_with_exit_code() {
  let (result, _) =
    with_io_output(|io| run_command(&["git", "clone", "not-a-real-repo"], None, true, io.output));
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Command failed"));
}
