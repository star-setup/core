use star_setup::ctx::{RunFlags, Runner};
use std::collections::VecDeque;
use std::{io::Write, path::Path};

pub struct MockRunner {
  pub calls: Vec<(Vec<String>, Option<std::path::PathBuf>)>,
  pub fail_on: Option<String>,
  pub capture_output: String,
  pub capture_responses: VecDeque<String>,
}

impl MockRunner {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      calls: vec![],
      fail_on: None,
      capture_output: String::new(),
      capture_responses: VecDeque::new(),
    }
  }
}

impl Runner for MockRunner {
  fn run(
    &mut self,
    cmd: &[&str],
    cwd: Option<&Path>,
    _flags: RunFlags,
    _output: &mut dyn Write,
  ) -> Result<(), String> {
    let cmd_vec: Vec<String> = cmd.iter().map(ToString::to_string).collect();
    self
      .calls
      .push((cmd_vec.clone(), cwd.map(Path::to_path_buf)));
    if let Some(fail) = &self.fail_on {
      if cmd_vec.contains(fail) {
        return Err(format!("MockRunner: forced failure on {fail}"));
      }
    }
    Ok(())
  }

  fn run_capture(&mut self, cmd: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    let cmd_vec: Vec<String> = cmd.iter().map(ToString::to_string).collect();
    self
      .calls
      .push((cmd_vec.clone(), cwd.map(Path::to_path_buf)));
    if let Some(fail) = &self.fail_on {
      if cmd_vec.contains(fail) {
        return Err(format!("MockRunner: forced failure on {fail}"));
      }
    }
    Ok(
      self
        .capture_responses
        .pop_front()
        .unwrap_or_else(|| self.capture_output.clone()),
    )
  }
}
