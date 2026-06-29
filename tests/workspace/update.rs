use crate::{
  common::{with_ctx, MockRunner},
  helpers::make_workspace,
};

#[test]
fn test_update_workspace_empty() {
  let (runner, _) = with_ctx(MockRunner::new(), |tmp, ctx| {
    let ws = make_workspace(tmp, vec![]);
    ws.update(ctx).unwrap();
  });
  assert!(runner.calls.is_empty());
}

#[test]
fn test_update_workspace_pulls_each_repo() {
  let (runner, _) = with_ctx(MockRunner::new(), |tmp, ctx| {
    let ws = make_workspace(
      tmp,
      vec![tmp.join("repos/user-lib1"), tmp.join("repos/user-lib2")],
    );
    ws.update(ctx).unwrap();
  });

  assert_eq!(runner.calls.len(), 2);
  assert!(runner
    .calls
    .iter()
    .all(|(cmd, _)| cmd[0] == "git" && cmd[1] == "pull"));
}

#[test]
fn test_update_workspace_continues_on_failure() {
  let mut runner = MockRunner::new();
  runner.fail_on = Some("pull".to_string());

  let (runner, output) = with_ctx(runner, |tmp, ctx| {
    let ws = make_workspace(
      tmp,
      vec![tmp.join("repos/user-lib1"), tmp.join("repos/user-lib2")],
    );
    let result = ws.update(ctx);
    assert!(result.is_err());
  });

  assert_eq!(runner.calls.len(), 2);
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Failed to update"));
}
