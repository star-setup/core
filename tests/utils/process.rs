use super::common::sink;
use star_setup::utils::process::run_command;

#[test]
fn test_run_command_errors_on_empty() {
  let mut output = sink();
  assert!(run_command(&[], None, false, &mut output).is_err());
}
