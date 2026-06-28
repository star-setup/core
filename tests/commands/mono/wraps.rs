use std::path::Path;

use super::super::common::{empty_input, make_io, sink};
use star_setup::{
  commands::{hoist_wraps, parse_project_name, parse_provide_pairs},
  ctx::IoCtx,
};
use tempfile::TempDir;

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

fn make_repo(project_name: &str) -> TempDir {
  let tmp = TempDir::new().unwrap();
  std::fs::write(
    tmp.path().join("meson.build"),
    format!("project('{project_name}', 'cpp')"),
  )
  .unwrap();
  tmp
}

fn run_hoist_test<F>(test_logic: F)
where
  F: FnOnce(&Path, &mut IoCtx<'_>),
{
  let repos_dir = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  test_logic(repos_dir.path(), &mut io);
}

#[test]
fn test_hoist_wraps_empty_repos() {
  run_hoist_test(|repos_dir, io| {
    let result = hoist_wraps(repos_dir, &[], io).unwrap();
    assert!(result.is_empty());
  });
}

#[test]
fn test_hoist_wraps_skips_repo_without_meson_build() {
  run_hoist_test(|repos_dir, io| {
    let repo = TempDir::new().unwrap();
    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io).unwrap();
    assert!(result.is_empty());
  });
}

#[test]
fn test_hoist_wraps_emits_wrap_without_provide() {
  run_hoist_test(|repos_dir, io| {
    let repo = make_repo("my-lib");
    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io).unwrap();

    assert!(result.contains_key("my_lib"));
    let wrap = repos_dir.join("my_lib.wrap");
    assert!(wrap.exists());

    let content = std::fs::read_to_string(&wrap).unwrap();
    assert!(content.contains("directory ="));
    assert!(!content.contains("[provide]"));
  });
}

#[test]
fn test_hoist_wraps_emits_wrap_with_provide() {
  run_hoist_test(|repos_dir, io| {
    let repo = make_repo("my-lib");
    let subprojects = repo.path().join("subprojects");
    std::fs::create_dir(&subprojects).unwrap();
    std::fs::write(
      subprojects.join("my_lib.wrap"),
      "[provide]\nmy_lib = my_lib_dep\n",
    )
    .unwrap();
    std::fs::write(subprojects.join("readme.txt"), "ignore me").unwrap();

    let result = hoist_wraps(repos_dir, &[repo.path().to_path_buf()], io).unwrap();

    assert!(result.contains_key("my_lib"));
    let wrap = repos_dir.join("my_lib.wrap");
    let content = std::fs::read_to_string(&wrap).unwrap();
    assert!(content.contains("[provide]"));
    assert!(content.contains("my_lib = my_lib_dep"));
  });
}
