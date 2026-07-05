use crate::common::{make_flags, with_io_dir};
use star_setup::commands::{hoist_wraps, parse_project_name, parse_provide_pairs};
use std::fs::{create_dir, read_to_string, write};
use tempfile::TempDir;

/* =====     PARSE_PROJECT_NAME     ===== */
#[test]
fn test_parse_project_name() {
  let cases = [
    ("project('my-lib', 'cpp')", Some("my_lib")),
    (r#"project("my-lib", "cpp")"#, Some("my_lib")),
    ("project('mylib', 'cpp')", Some("mylib")),
    ("myproject('mylib', 'cpp')", None),
    ("cmake_minimum_required(VERSION 3.20)", None),
    ("project(mylib, cpp)", None),
  ];

  for (input, expected) in cases {
    assert_eq!(
      parse_project_name(input),
      expected.map(String::from),
      "Failed on input: {input}"
    );
  }
}

/* =====     PARSE_PROVIDE_PAIRS     ===== */
#[test]
fn test_parse_provide_pairs_basic() {
  let content = "[provide]\nmy_lib = my_lib_dep\n";
  let pairs = parse_provide_pairs(content);
  assert_eq!(pairs.get("my_lib"), Some(&"my_lib_dep".to_string()));
}

#[test]
fn test_parse_provide_pairs_multiple() {
  let content = "[provide]\nfoo = foo_dep\nbar = bar_dep\n";
  let pairs = parse_provide_pairs(content);
  assert_eq!(pairs.len(), 2);
}

#[test]
fn test_parse_provide_pairs_ignores_other_sections() {
  let content = "[wrap-file]\nurl = http://example.com\n\n[provide]\nmy_lib = my_lib_dep\n";
  let pairs = parse_provide_pairs(content);
  assert_eq!(pairs.len(), 1);
  assert!(pairs.contains_key("my_lib"));
}

#[test]
fn test_parse_provide_pairs_empty() {
  let pairs = parse_provide_pairs("");
  assert!(pairs.is_empty());
}

#[test]
fn test_parse_provide_pairs_no_provide_section() {
  let content = "[wrap-file]\nurl = http://example.com\n";
  let pairs = parse_provide_pairs(content);
  assert!(pairs.is_empty());
}

/* =====     HELPERS     ===== */
fn make_repo(project_name: &str) -> TempDir {
  let tmp = TempDir::new().unwrap();
  write(
    tmp.path().join("meson.build"),
    format!("project('{project_name}', 'cpp')"),
  )
  .unwrap();
  tmp
}

/* =====     HOIST_WRAPS     ===== */
#[test]
fn test_hoist_wraps_empty_repos() {
  with_io_dir(|repos_dir, io| {
    let result = hoist_wraps(repos_dir, &[], io, make_flags()).unwrap();
    assert!(result.is_empty());
  });
}

#[test]
fn test_hoist_wraps_skips_repo_without_meson_build() {
  with_io_dir(|repos_dir, io| {
    let repo = TempDir::new().unwrap();
    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io, make_flags()).unwrap();
    assert!(result.is_empty());
  });
}

#[test]
fn test_hoist_wraps_emits_wrap_without_provide() {
  with_io_dir(|repos_dir, io| {
    let repo = make_repo("my-lib");
    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io, make_flags()).unwrap();

    assert!(result.contains_key("my_lib"));
    let wrap = repos_dir.join("my_lib.wrap");
    assert!(wrap.exists());

    let content = read_to_string(&wrap).unwrap();
    assert!(content.contains("directory ="));
    assert!(!content.contains("[provide]"));
  });
}

#[test]
fn test_hoist_wraps_emits_wrap_with_provide() {
  with_io_dir(|repos_dir, io| {
    let repo = make_repo("my-lib");
    let subprojects = repo.path().join("subprojects");
    create_dir(&subprojects).unwrap();
    write(
      subprojects.join("my_lib.wrap"),
      "[provide]\nmy_lib = my_lib_dep\n",
    )
    .unwrap();
    write(subprojects.join("readme.txt"), "ignore me").unwrap();

    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io, make_flags()).unwrap();

    assert!(result.contains_key("my_lib"));
    let wrap = repos_dir.join("my_lib.wrap");
    let content = read_to_string(&wrap).unwrap();
    assert!(content.contains("[provide]"));
    assert!(content.contains("my_lib = my_lib_dep"));
  });
}
