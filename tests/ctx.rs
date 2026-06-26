#[test]
fn test_process_runner_runs_command() {
  use star_setup::ctx::{IoCtx, ProcessRunner, Runner};
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut runner = ProcessRunner;
  let mut io = IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: false,
    timing: false,
  };
  assert!(runner.run(&["git", "--version"], None, &mut io).is_ok());
}
