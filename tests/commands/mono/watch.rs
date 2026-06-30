use crate::common::{make_flags, with_io_dir, with_io_output};
use star_setup::commands::mono::watch::generate_watch_scripts;

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
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
    make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);
    let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
    generate_watch_scripts(tmp_path, &repos_path, &repos, io, make_flags()).unwrap();
    assert!(tmp_path.join("watch.ps1").exists());
    assert!(tmp_path.join("watch.sh").exists());
  });
}

#[test]
fn test_generate_watch_scripts_prefers_watch_script() {
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
    make_repo_with_scripts(
      &repos_path,
      "user-lib1",
      r#""build": "tsdown", "watch": "tsdown --watch""#,
    );
    let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
    generate_watch_scripts(tmp_path, &repos_path, &repos, io, make_flags()).unwrap();
    let ps1 = std::fs::read_to_string(tmp_path.join("watch.ps1")).unwrap();
    assert!(ps1.contains("run watch"));
    assert!(!ps1.contains("run build"));
  });
}

#[test]
fn test_generate_watch_scripts_falls_back_to_build() {
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
    make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);
    let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
    generate_watch_scripts(tmp_path, &repos_path, &repos, io, make_flags()).unwrap();
    let ps1 = std::fs::read_to_string(tmp_path.join("watch.ps1")).unwrap();
    assert!(ps1.contains("run build -- --watch"));
  });
}

#[test]
fn test_generate_watch_scripts_empty_libs() {
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
    let repos = vec!["user/game".to_string()];
    generate_watch_scripts(tmp_path, &repos_path, &repos, io, make_flags()).unwrap();
    assert!(!tmp_path.join("watch.ps1").exists());
    assert!(!tmp_path.join("watch.sh").exists());
  });
}

#[test]
fn test_generate_watch_scripts_no_scripts_field() {
  with_io_dir(|tmp_path, io| {
    let repos_path = tmp_path.join("repos");
    std::fs::create_dir_all(repos_path.join("user-lib1")).unwrap();
    std::fs::write(
      repos_path.join("user-lib1").join("package.json"),
      r#"{"name": "@user/lib1"}"#,
    )
    .unwrap();
    let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
    generate_watch_scripts(tmp_path, &repos_path, &repos, io, make_flags()).unwrap();
    let ps1 = std::fs::read_to_string(tmp_path.join("watch.ps1")).unwrap();
    assert!(!ps1.contains("user-lib1"));
  });
}

#[test]
fn test_generate_watch_scripts_verbose_output() {
  let ((), out) = with_io_output(|io| {
    let tmp = tempfile::TempDir::new().unwrap();
    let repos_path = tmp.path().join("repos");
    make_repo_with_scripts(&repos_path, "user-lib1", r#""build": "tsdown""#);
    let repos = vec!["user/game".to_string(), "user/lib1".to_string()];
    let flags = star_setup::ctx::RunFlags {
      verbose: true,
      timing: false,
      dry_run: false,
    };
    generate_watch_scripts(tmp.path(), &repos_path, &repos, io, flags).unwrap();
  });
  assert!(out.contains("Watching 1 libraries:"));
  assert!(out.contains("user-lib1"));
}
