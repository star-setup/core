use super::io::{empty_input, make_io, sink};
use star_setup::ctx::{IoCtx, RunCtx, Runner};
use std::path::Path;
use tempfile::TempDir;

#[allow(dead_code)]
pub fn with_io(f: impl FnOnce(&mut IoCtx<'_>)) {
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  f(&mut io);
}

#[allow(dead_code)]
pub fn with_io_dir(f: impl FnOnce(&Path, &mut IoCtx<'_>)) {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  f(tmp.path(), &mut io);
}

#[allow(dead_code)]
pub fn with_ctx<R: Runner>(
  mut runner: R,
  f: impl FnOnce(&Path, &mut RunCtx<'_, '_>),
) -> (R, Vec<u8>) {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = Vec::new();
  {
    let mut ctx = RunCtx {
      io: make_io(&mut input, &mut output),
      runner: &mut runner,
    };
    f(tmp.path(), &mut ctx);
  }
  (runner, output)
}

#[allow(dead_code)]
pub fn with_runner_ctx<R: Runner>(runner: R, f: impl FnOnce(&Path, &mut RunCtx<'_, '_>)) -> R {
  with_ctx(runner, f).0
}
