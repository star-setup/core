use crate::common::{make_flags, with_io_dir, with_io_input_output};
use star_setup::commands::mono::display::{print_setup_complete, resolve_setup_paths};
use std::collections::HashMap;

#[test]
fn test_print_setup_complete_no_map() {
  let ((), out) = with_io_input_output(b"", |io| {
    with_io_dir(|tmp_path, _| {
      let paths = resolve_setup_paths(
        None::<&HashMap<String, String>>,
        tmp_path,
        &tmp_path.join("build"),
        "user/repo",
      );
      print_setup_complete(&paths, std::time::Instant::now(), io, &mut make_flags());
    });
  });

  assert!(out.contains("Setup complete"));
  assert!(out.contains("Build output in:"));
}

#[test]
fn test_print_setup_complete_with_map() {
  let ((), out) = with_io_input_output(b"", |io| {
    with_io_dir(|tmp_path, _| {
      let mut map = HashMap::new();
      map.insert("my_lib".to_string(), "user-repo".to_string());

      let paths = resolve_setup_paths(Some(&map), tmp_path, &tmp_path.join("build"), "user/repo");
      print_setup_complete(&paths, std::time::Instant::now(), io, &mut make_flags());
    });
  });

  assert!(out.contains("Setup complete"));
  assert!(out.contains("Executable:"));
}

#[test]
fn test_print_setup_complete_timing() {
  let ((), out) = with_io_input_output(b"", |io| {
    with_io_dir(|tmp_path, _| {
      let paths = resolve_setup_paths(
        None::<&HashMap<String, String>>,
        tmp_path,
        &tmp_path.join("build"),
        "user/repo",
      );
      print_setup_complete(
        &paths,
        std::time::Instant::now(),
        io,
        &mut star_setup::ctx::RunFlags {
          timing: true,
          verbose: false,
          dry_run: false,
        },
      );
    });
  });
  assert!(out.contains("[timing] Total:"));
}
