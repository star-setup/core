use super::common::make_io;
use star_setup::{ctx::IoCtx, utils::check_prerequisites};

#[test]
fn test_check_prerequisites_succeeds_with_tools_present() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = make_io(&mut input, &mut output);
  assert!(check_prerequisites(&mut io).is_ok());
}

#[test]
fn test_check_prerequisites_verbose_outputs_found() {
  let mut input = b"".as_ref();
  let mut output = Vec::new();
  let mut io = IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: true,
    timing: false,
  };
  check_prerequisites(&mut io).unwrap();
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Found git"));
  assert!(out.contains("Found cmake"));
  assert!(out.contains("Found meson"));
}
