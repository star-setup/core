#![allow(dead_code)]

pub fn sink() -> Vec<u8> {
  vec![]
}

pub fn empty_input() -> &'static [u8] {
  b""
}

pub fn make_io<'a>(
  input: &'a mut dyn std::io::BufRead,
  output: &'a mut dyn std::io::Write,
) -> star_setup::ctx::IoCtx<'a> {
  star_setup::ctx::IoCtx {
    input,
    output,
    verbose: false,
    timing: false,
  }
}
