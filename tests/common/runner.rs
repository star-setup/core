use star_setup::ctx::{IoCtx, Runner};
use std::path::Path;

#[allow(dead_code)]
pub struct MockRunner {
  pub calls: Vec<(Vec<String>, Option<std::path::PathBuf>)>,
  pub fail_on: Option<String>,
}

impl MockRunner {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self { calls: vec![], fail_on: None }
  }
}

impl Runner for MockRunner {
  fn run(&mut self, cmd: &[&str], cwd: Option<&Path>, _io: &mut IoCtx<'_>) -> Result<(), String> {
    let cmd_vec: Vec<String> = cmd.iter().map(ToString::to_string).collect();
    if let Some(fail) = &self.fail_on {
      if cmd_vec.contains(fail) {
        return Err(format!("MockRunner: forced failure on {fail}"));
      }
    }
    self.calls.push((cmd_vec, cwd.map(Path::to_path_buf)));
    Ok(())
  }
}
