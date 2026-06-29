use star_setup::ctx::{IoCtx, RunFlags};

pub fn sink() -> Vec<u8> {
  vec![]
}

pub fn empty_input() -> &'static [u8] {
  b""
}

pub fn make_io<'a>(
  input: &'a mut dyn std::io::BufRead,
  output: &'a mut dyn std::io::Write,
) -> IoCtx<'a> {
  IoCtx { input, output }
}

pub fn make_flags() -> RunFlags {
  RunFlags {
    verbose: false,
    timing: false,
    dry_run: false,
  }
}
