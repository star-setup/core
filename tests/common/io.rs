use star_setup::ctx::{IoCtx, RunFlags};
use std::io::{BufRead, Write};

pub fn sink() -> Vec<u8> {
  vec![]
}

pub fn empty_input() -> &'static [u8] {
  b""
}

pub fn make_io<'a>(input: &'a mut dyn BufRead, output: &'a mut dyn Write) -> IoCtx<'a> {
  IoCtx { input, output }
}

pub fn make_flags() -> RunFlags {
  RunFlags {
    verbose: false,
    timing: false,
    dry_run: false,
  }
}
