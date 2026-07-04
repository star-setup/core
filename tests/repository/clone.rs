use crate::common::{with_ctx, with_ctx_input, with_ctx_runner, MockRunner};
use star_setup::{
  ctx::ProcessRunner,
  repository::{clone_repo, clone_repos, ExistsAction},
};
use std::fs::{create_dir_all, write};
use tempfile::TempDir;

/* =====     CLONE_REPO     ===== */
#[test]
fn test_clone_skips_existing_directory() {
  with_ctx_runner(ProcessRunner, |tmp_path, ctx| {
    let repo_dir = tmp_path.join("owner-repo");
    create_dir_all(repo_dir.join(".git")).unwrap();

    let result = clone_repo(
      "owner/repo",
      tmp_path,
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    );
    assert!(result.is_ok());
    assert!(repo_dir.exists());
  });
}

#[test]
fn test_clone_repo_calls_git_clone() {
  let tmp = TempDir::new().unwrap();

  let runner = with_ctx_runner(MockRunner::new(), |_, ctx| {
    clone_repo(
      "user/repo",
      tmp.path(),
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    )
    .unwrap();
  });

  assert_eq!(runner.calls.len(), 1);
  let (cmd, cwd) = &runner.calls[0];
  assert_eq!(cmd[0], "git");
  assert_eq!(cmd[1], "clone");
  assert!(cmd[2].contains("user/repo"));
  assert_eq!(cwd.as_deref(), Some(tmp.path()));
}

#[test]
fn test_clone_repo_invalid_dir_decline_skips() {
  let (runner, output) = with_ctx_input(b"n\n", MockRunner::new(), |tmp_path, ctx| {
    let repo_dir = tmp_path.join("user-repo");
    create_dir_all(&repo_dir).unwrap();
    write(repo_dir.join("keep.txt"), "data").unwrap();
    clone_repo(
      "user/repo",
      tmp_path,
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    )
    .unwrap();
    assert!(repo_dir.join("keep.txt").exists());
  });
  assert!(runner.calls.is_empty());
  assert!(String::from_utf8(output).unwrap().contains("Skipping"));
}

#[test]
fn test_clone_repo_invalid_dir_accept_removes_and_clones() {
  let (runner, _) = with_ctx_input(b"y\n", MockRunner::new(), |tmp_path, ctx| {
    let repo_dir = tmp_path.join("user-repo");
    create_dir_all(&repo_dir).unwrap();
    write(repo_dir.join("stale.txt"), "old").unwrap();
    clone_repo(
      "user/repo",
      tmp_path,
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    )
    .unwrap();
    assert!(!repo_dir.exists()); // removed; mock clone doesn't recreate
  });
  assert_eq!(
    runner.calls.iter().filter(|(c, _)| c[1] == "clone").count(),
    1
  );
}

#[test]
fn test_clone_repo_invalid_dir_dry_run_would_remove() {
  let (_, output) = with_ctx_input(b"y\n", MockRunner::new(), |tmp_path, ctx| {
    ctx.flags.dry_run = true;
    let repo_dir = tmp_path.join("user-repo");
    create_dir_all(&repo_dir).unwrap();
    write(repo_dir.join("keep.txt"), "data").unwrap();
    clone_repo(
      "user/repo",
      tmp_path,
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    )
    .unwrap();
    assert!(repo_dir.join("keep.txt").exists()); // NOT removed in dry-run
  });
  assert!(String::from_utf8(output)
    .unwrap()
    .contains("Would remove directory:"));
}

#[test]
fn test_clone_repo_empty_dir_clones_without_prompt() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    create_dir_all(tmp_path.join("user-repo")).unwrap();
    clone_repo(
      "user/repo",
      tmp_path,
      false,
      |_| Ok(ExistsAction::Skip),
      ctx,
    )
    .unwrap();
  });
  assert_eq!(
    runner.calls.iter().filter(|(c, _)| c[1] == "clone").count(),
    1
  );
}

/* =====     CLONE_REPOS     ===== */
#[test]
fn test_clone_repos_calls_clone_for_each_repo() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 2);
  assert!(runner
    .calls
    .iter()
    .all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "clone"));
}

#[test]
fn test_clone_repos_empty() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    clone_repos(&[], tmp_path, false, false, ctx).unwrap();
  });

  assert!(runner.calls.is_empty());
}

#[test]
fn test_clone_repos_per_repo_lines_always_shown() {
  let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Cloning repositories"));
  assert!(out.contains("Cloning user-repo1"));
  assert!(!out.contains("Finished cloning (2 repositories)"));

  let (_, output) = with_ctx(MockRunner::new(), |tmp_path, ctx| {
    ctx.flags.verbose = true;
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Cloning repositories"));
  assert!(out.contains("Cloning user-repo1"));
  assert!(out.contains("Finished cloning (2 repositories)"));
}

#[test]
fn test_clone_repos_yes_updates_existing_without_prompt() {
  let runner = with_ctx_runner(MockRunner::new(), |tmp_path, ctx| {
    create_dir_all(tmp_path.join("user-repo1").join(".git")).unwrap();
    clone_repos(&["user/repo1".to_string()], tmp_path, false, true, ctx).unwrap();
  });
  assert_eq!(
    runner.calls.iter().filter(|(c, _)| c[1] == "pull").count(),
    1
  );
  assert_eq!(
    runner.calls.iter().filter(|(c, _)| c[1] == "clone").count(),
    0
  );
}

#[test]
fn test_clone_repos_prompts_per_existing_repo() {
  let (runner, _) = with_ctx_input(b"y\nn\n", MockRunner::new(), |tmp_path, ctx| {
    create_dir_all(tmp_path.join("user-repo1").join(".git")).unwrap();
    create_dir_all(tmp_path.join("user-repo2").join(".git")).unwrap();
    let repos = vec!["user/repo1".to_string(), "user/repo2".to_string()];
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });
  let pulls: Vec<_> = runner
    .calls
    .iter()
    .filter(|(c, _)| c[1] == "pull")
    .collect();
  assert_eq!(pulls.len(), 1);
  assert!(pulls[0].1.as_deref().unwrap().ends_with("user-repo1")); // y→first, n→second
}

#[test]
fn test_clone_repos_yes_all_updates_remaining() {
  // 5 existing (>= threshold), a SINGLE "a" must drive all five — a broken latch
  // would EOF on repo 2 and panic
  let (runner, _) = with_ctx_input(b"a\n", MockRunner::new(), |tmp_path, ctx| {
    let repos: Vec<String> = (1..=5).map(|i| format!("user/repo{i}")).collect();
    for i in 1..=5 {
      create_dir_all(tmp_path.join(format!("user-repo{i}")).join(".git")).unwrap();
    }
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });
  assert_eq!(
    runner.calls.iter().filter(|(c, _)| c[1] == "pull").count(),
    5
  );
}

#[test]
fn test_clone_repos_no_all_skips_remaining() {
  let (runner, _) = with_ctx_input(b"s\n", MockRunner::new(), |tmp_path, ctx| {
    let repos: Vec<String> = (1..=5).map(|i| format!("user/repo{i}")).collect();
    for i in 1..=5 {
      create_dir_all(tmp_path.join(format!("user-repo{i}")).join(".git")).unwrap();
    }
    clone_repos(&repos, tmp_path, false, false, ctx).unwrap();
  });
  assert!(runner.calls.is_empty());
}
