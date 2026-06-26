use super::common::sink;
use star_setup::utils::process::run_command;

#[test]
fn test_run_command_errors_on_empty() {
  let mut output = sink();
  assert!(run_command(&[], None, false, &mut output).is_err());
}

#[test]
fn test_run_command_verbose_outputs_command() {
  let mut output = Vec::new();
  star_setup::utils::run_command(&["git", "--version"], None, true, &mut output).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Running: git --version"));
}

#[test]
fn test_run_command_verbose_outputs_cwd() {
  let tmp = tempfile::TempDir::new().unwrap();
  let mut output = Vec::new();
  star_setup::utils::run_command(&["git", "--version"], Some(tmp.path()), true, &mut output)
    .unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("in directory:"));
}

#[test]
fn test_run_command_fails_with_stderr() {
  let mut output = Vec::new();
  let result = star_setup::utils::run_command(
    &["git", "clone", "not-a-real-repo"],
    None,
    false,
    &mut output,
  );
  assert!(result.is_err());
}

#[test]
fn test_run_command_fails_no_stderr() {
  let mut output = Vec::new();
  let result =
    star_setup::utils::run_command(&["git", "invalid-command-xyz"], None, false, &mut output);
  assert!(result.is_err());
}

#[test]
fn test_run_command_verbose_fails_with_exit_code() {
  let mut output = Vec::new();
  let result = star_setup::utils::run_command(
    &["git", "clone", "not-a-real-repo"],
    None,
    true,
    &mut output,
  );
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Command failed"));
}
