use crate::common::with_io_output;
use star_setup::{build::read_package_json, ctx::RunFlags};
use std::fs::{create_dir_all, write};
use tempfile::TempDir;

fn flags(verbose: bool) -> RunFlags {
  RunFlags {
    verbose,
    timing: false,
    dry_run: false,
  }
}

#[test]
fn test_read_package_json_valid_returns_json_silently() {
  let tmp = TempDir::new().unwrap();
  let repo = tmp.path().join("user-lib1");
  create_dir_all(&repo).unwrap();
  write(repo.join("package.json"), r#"{"name": "@user/lib1"}"#).unwrap();

  let (result, out) = with_io_output(|io| read_package_json(&repo, "skipping", io, flags(true)));
  assert!(result.is_some());
  assert_eq!(out, "");
}

#[test]
fn test_read_package_json_missing_file_warns_when_verbose() {
  let tmp = TempDir::new().unwrap();
  let repo = tmp.path().join("user-lib1");
  let (result, out) =
    with_io_output(|io| read_package_json(&repo, "skipping override", io, flags(true)));
  assert!(result.is_none());
  let pkg = repo.join("package.json");
  assert!(out.contains(&pkg.display().to_string()));
  assert!(out.contains("skipping override"));
}

#[test]
fn test_read_package_json_missing_file_silent_when_quiet() {
  let tmp = TempDir::new().unwrap();
  let (result, out) =
    with_io_output(|io| read_package_json(tmp.path(), "skipping", io, flags(false)));
  assert!(result.is_none());
  assert_eq!(out, "");
}

#[test]
fn test_read_package_json_malformed_warns_when_verbose() {
  let tmp = TempDir::new().unwrap();
  let repo = tmp.path().join("user-lib1");
  create_dir_all(&repo).unwrap();
  write(repo.join("package.json"), "{ not json").unwrap();

  let (result, out) = with_io_output(|io| read_package_json(&repo, "skipping", io, flags(true)));
  assert!(result.is_none());
  let pkg = repo.join("package.json");
  assert!(out.contains(&pkg.display().to_string()));
  assert!(out.contains("skipping"));
}
