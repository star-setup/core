use crate::common::io::{empty_input, make_flags, make_io, sink};
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
pub fn with_io_input<T>(input: &[u8], f: impl FnOnce(&mut IoCtx<'_>) -> T) -> T {
  let mut input_slice = input;
  let mut output = sink();
  let mut io = make_io(&mut input_slice, &mut output);
  f(&mut io)
}

#[allow(dead_code)]
pub fn with_io_output<T>(f: impl FnOnce(&mut IoCtx<'_>) -> T) -> (T, String) {
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  let result = f(&mut io);
  (result, String::from_utf8(output).unwrap_or_default())
}

#[allow(dead_code)]
pub fn with_io_input_output<T>(input: &[u8], f: impl FnOnce(&mut IoCtx<'_>) -> T) -> (T, String) {
  let mut input_slice = input;
  let mut output = Vec::new();
  let mut io = make_io(&mut input_slice, &mut output);
  let result = f(&mut io);
  (result, String::from_utf8(output).unwrap_or_default())
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
      flags: make_flags(),
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
