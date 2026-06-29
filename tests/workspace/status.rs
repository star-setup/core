use crate::{
  common::{with_ctx, MockRunner},
  helpers::make_workspace,
};

#[test]
fn test_status_workspace_empty() {
  let (_, output) = with_ctx(MockRunner::new(), |tmp, ctx| {
    let ws = make_workspace(tmp, vec![]);
    ws.status(false, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Workspace status:"));
}

#[test]
fn test_status_workspace_shows_repos() {
  let mut runner = MockRunner::new();
  runner.capture_responses.push_back("main".to_string());
  runner.capture_responses.push_back(String::new());

  let (_, output) = with_ctx(runner, |tmp, ctx| {
    let ws = make_workspace(tmp, vec![tmp.join("repos/user-lib1")]);
    ws.status(false, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("user-lib1"));
  assert!(out.contains("clean"));
}

#[test]
fn test_status_workspace_with_fetch() {
  let mut runner = MockRunner::new();
  runner.capture_responses.push_back("main".to_string());
  runner.capture_responses.push_back(String::new());
  runner.capture_responses.push_back("2".to_string());
  runner.capture_responses.push_back("1".to_string());

  let (runner, output) = with_ctx(runner, |tmp, ctx| {
    let ws = make_workspace(tmp, vec![tmp.join("repos/user-lib1")]);
    ws.status(true, ctx).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("↑2 ↓1"));
  assert!(runner
    .calls
    .iter()
    .any(|(cmd, _)| cmd[0] == "git" && cmd[1] == "fetch"));
}
