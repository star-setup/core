use crate::common::with_io_output;
use star_setup::{build::resolve_dev_command, ctx::RunFlags};
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
fn test_resolve_dev_command_uses_dev_script() {
  let tmp = TempDir::new().unwrap();
  write(
    tmp.path().join("package.json"),
    r#"{"scripts": {"dev": "vite"}}"#,
  )
  .unwrap();
  let (result, _) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(false)));
  assert_eq!(result, Some("npm run dev".to_string()));
}

#[test]
fn test_resolve_dev_command_falls_back_to_vercel_dev_with_vercel_json() {
  let tmp = TempDir::new().unwrap();
  write(tmp.path().join("package.json"), r#"{"scripts": {}}"#).unwrap();
  write(tmp.path().join("vercel.json"), "{}").unwrap();
  let (result, _) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(false)));
  assert_eq!(result, Some("vercel dev".to_string()));
}

#[test]
fn test_resolve_dev_command_falls_back_to_vercel_dev_with_api_dir() {
  let tmp = TempDir::new().unwrap();
  write(tmp.path().join("package.json"), r#"{"scripts": {}}"#).unwrap();
  create_dir_all(tmp.path().join("api")).unwrap();
  let (result, _) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(false)));
  assert_eq!(result, Some("vercel dev".to_string()));
}

#[test]
fn test_resolve_dev_command_falls_back_to_vercel_dev_with_vercel_node_dependency() {
  let tmp = TempDir::new().unwrap();
  write(
    tmp.path().join("package.json"),
    r#"{"scripts": {}, "dependencies": {"@vercel/node": "^5.0.0"}}"#,
  )
  .unwrap();
  let (result, _) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(false)));
  assert_eq!(result, Some("vercel dev".to_string()));
}

#[test]
fn test_resolve_dev_command_falls_back_to_vercel_dev_with_vercel_node_dev_dependency() {
  let tmp = TempDir::new().unwrap();
  write(
    tmp.path().join("package.json"),
    r#"{"scripts": {}, "devDependencies": {"@vercel/node": "^5.0.0"}}"#,
  )
  .unwrap();
  let (result, _) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(false)));
  assert_eq!(result, Some("vercel dev".to_string()));
}

#[test]
fn test_resolve_dev_command_none_when_no_signals() {
  let tmp = TempDir::new().unwrap();
  write(
    tmp.path().join("package.json"),
    r#"{"scripts": {"typecheck": "tsc"}}"#,
  )
  .unwrap();
  let (result, out) = with_io_output(|io| resolve_dev_command(tmp.path(), io, flags(true)));
  assert!(result.is_none());
  assert!(out.contains("No dev script found"));
}
