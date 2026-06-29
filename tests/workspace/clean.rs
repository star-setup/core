use crate::{
  common::{with_ctx, MockRunner},
  helpers::make_workspace,
};
use std::fs;

#[test]
fn test_workspace_clean_no_build_dir() {
  let (_, output) = with_ctx(MockRunner::new(), |tmp, ctx| {
    let ws = make_workspace(tmp, vec![]);
    ws.clean(ctx).unwrap();
  });
  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("does not exist"));
}

#[test]
fn test_workspace_clean_removes_build_dir() {
  with_ctx(MockRunner::new(), |tmp, ctx| {
    let ws = make_workspace(tmp, vec![]);
    fs::create_dir_all(&ws.build_path).unwrap();
    fs::write(ws.build_path.join("dummy.txt"), "").unwrap();
    ws.clean(ctx).unwrap();
    assert!(!ws.build_path.exists());
  });
}

#[test]
fn test_workspace_clean_dry_run() {
  let (_, output) = with_ctx(MockRunner::new(), |tmp, ctx| {
    ctx.flags.dry_run = true;

    let ws = make_workspace(tmp, vec![]);
    fs::create_dir_all(&ws.build_path).unwrap();

    ws.clean(ctx).unwrap();

    assert!(ws.build_path.exists());
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Would remove directory:"));
}
