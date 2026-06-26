use super::common::{empty_input, make_io, sink};
use star_setup::commands::mono::display::print_setup_complete;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_print_setup_complete_no_map() {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  print_setup_complete(
    None::<HashMap<String, String>>.as_ref(),
    tmp.path(),
    &tmp.path().join("build"),
    "user/repo",
    std::time::Instant::now(),
    &mut io,
  );

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Setup complete"));
  assert!(out.contains("Build output in:"));
}

#[test]
fn test_print_setup_complete_with_map() {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  let mut map = HashMap::new();
  map.insert("my_lib".to_string(), "user-repo".to_string());

  print_setup_complete(
    Some(&map),
    tmp.path(),
    &tmp.path().join("build"),
    "user/repo",
    std::time::Instant::now(),
    &mut io,
  );

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Setup complete"));
  assert!(out.contains("Executable:"));
}

#[test]
fn test_print_setup_complete_timing() {
  let tmp = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = star_setup::ctx::IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: false,
    timing: true,
  };

  print_setup_complete(
    None::<HashMap<String, String>>.as_ref(),
    tmp.path(),
    &tmp.path().join("build"),
    "user/repo",
    std::time::Instant::now(),
    &mut io,
  );

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("[timing] Total:"));
}
