use super::super::common::{empty_input, make_io, sink};
use star_setup::commands::mono::watch::generate_watch_scripts;
use tempfile::TempDir;

fn make_repo_with_scripts(repos_path: &std::path::Path, dir: &str, scripts: &str) {
  std::fs::create_dir_all(repos_path.join(dir)).unwrap();
  std::fs::write(
    repos_path.join(dir).join("package.json"),
    format!(r#"{{"name": "@user/{dir}", "scripts": {{{scripts}}}}}"#),
  )
  .unwrap();
}

#[test]
fn test_generate_watch_scripts_creates_files() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");
  make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);

  let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  assert!(tmp.path().join("watch.ps1").exists());
  assert!(tmp.path().join("watch.sh").exists());
}

#[test]
fn test_generate_watch_scripts_prefers_watch_script() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");
  make_repo_with_scripts(
    &repos_path,
    "user-lib1",
    r#""build": "tsdown", "watch": "tsdown --watch""#,
  );

  let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  let ps1 = std::fs::read_to_string(tmp.path().join("watch.ps1")).unwrap();
  assert!(ps1.contains("run watch"));
  assert!(!ps1.contains("run build"));
}

#[test]
fn test_generate_watch_scripts_falls_back_to_build() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");
  make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);

  let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  let ps1 = std::fs::read_to_string(tmp.path().join("watch.ps1")).unwrap();
  assert!(ps1.contains("run build -- --watch"));
}

#[test]
fn test_generate_watch_scripts_empty_libs() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");

  let repos = vec!["user/game".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  assert!(!tmp.path().join("watch.ps1").exists());
  assert!(!tmp.path().join("watch.sh").exists());
}

#[test]
fn test_generate_watch_scripts_no_scripts_field() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");
  std::fs::create_dir_all(repos_path.join("user-lib1")).unwrap();
  std::fs::write(
    repos_path.join("user-lib1").join("package.json"),
    r#"{"name": "@user/lib1"}"#,
  )
  .unwrap();

  let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
  let mut input = empty_input();
  let mut output = sink();
  let mut io = make_io(&mut input, &mut output);

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  let ps1 = std::fs::read_to_string(tmp.path().join("watch.ps1")).unwrap();
  assert!(!ps1.contains("user-lib1"));
}

#[test]
fn test_generate_watch_scripts_verbose_output() {
  let tmp = TempDir::new().unwrap();
  let repos_path = tmp.path().join("repos");
  make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);

  let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
  let mut input = empty_input();
  let mut output = Vec::new();
  let mut io = star_setup::ctx::IoCtx {
    input: &mut input,
    output: &mut output,
    verbose: true,
    timing: false,
    dry_run: false,
  };

  generate_watch_scripts(tmp.path(), &repos_path, &repos, &mut io).unwrap();

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Watching 1 libraries:"));
  assert!(out.contains("user-lib1"));
}
