use super::common::{empty_input, make_io, sink};
use star_setup::commands::{hoist_wraps, parse_project_name, parse_provide_pairs};
use tempfile::TempDir;

#[test]
fn test_parse_project_name_single_quoted() {
  assert_eq!(
    parse_project_name("project('my-lib', 'cpp')"),
    Some("my_lib".to_string())
  );
}

#[test]
fn test_parse_project_name_double_quoted() {
  assert_eq!(
    parse_project_name(r#"project("my-lib", "cpp")"#),
    Some("my_lib".to_string())
  );
}

#[test]
fn test_parse_project_name_no_hyphens() {
  assert_eq!(
    parse_project_name("project('mylib', 'cpp')"),
    Some("mylib".to_string())
  );
}

#[test]
fn test_parse_project_name_prefix_guard() {
  assert_eq!(parse_project_name("myproject('mylib', 'cpp')"), None);
}

#[test]
fn test_parse_project_name_missing() {
  assert_eq!(
    parse_project_name("cmake_minimum_required(VERSION 3.20)"),
    None
  );
}

#[test]
fn test_parse_project_name_no_quotes() {
  assert_eq!(parse_project_name("project(mylib, cpp)"), None);
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

#[test]
fn test_hoist_wraps_empty_repos() {
  let repos_dir = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  let result = hoist_wraps(repos_dir.path(), &[], &mut io).unwrap();
  assert!(result.is_empty());
}

#[test]
fn test_hoist_wraps_skips_repo_without_meson_build() {
  let repos_dir = TempDir::new().unwrap();
  let repo = TempDir::new().unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  let result = hoist_wraps(repos_dir.path(), &[repo.path().to_path_buf()], &mut io).unwrap();
  assert!(result.is_empty());
}

#[test]
fn test_hoist_wraps_emits_wrap_without_provide() {
  let repos_dir = TempDir::new().unwrap();
  let repo = make_repo("my-lib");
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  let result = hoist_wraps(repos_dir.path(), &[repo.path().to_path_buf()], &mut io).unwrap();
  assert!(result.contains_key("my_lib"));
  let wrap = repos_dir.path().join("my_lib.wrap");
  assert!(wrap.exists());
  let content = std::fs::read_to_string(&wrap).unwrap();
  assert!(content.contains("directory ="));
  assert!(!content.contains("[provide]"));
}

#[test]
fn test_hoist_wraps_emits_wrap_with_provide() {
  let repos_dir = TempDir::new().unwrap();
  let repo = make_repo("my-lib");
  let subprojects = repo.path().join("subprojects");
  std::fs::create_dir(&subprojects).unwrap();
  std::fs::write(
    subprojects.join("my_lib.wrap"),
    "[provide]\nmy_lib = my_lib_dep\n",
  )
  .unwrap();
  std::fs::write(subprojects.join("readme.txt"), "ignore me").unwrap();
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);
  let result = hoist_wraps(repos_dir.path(), &[repo.path().to_path_buf()], &mut io).unwrap();
  assert!(result.contains_key("my_lib"));
  let wrap = repos_dir.path().join("my_lib.wrap");
  let content = std::fs::read_to_string(&wrap).unwrap();
  assert!(content.contains("[provide]"));
  assert!(content.contains("my_lib = my_lib_dep"));
}
