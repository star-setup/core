use crate::common::{with_ctx, MockRunner};
use star_setup::utils::check_prerequisites;

#[test]
fn test_check_prerequisites_succeeds_with_tools_present() {
  with_ctx(MockRunner::new(), |_, ctx| {
    assert!(check_prerequisites(&mut ctx.io, ctx.flags).is_ok());
  });
}

#[test]
fn test_check_prerequisites_verbose_outputs_found() {
  let (_, output) = with_ctx(MockRunner::new(), |_, ctx| {
    ctx.flags.verbose = true;
    check_prerequisites(&mut ctx.io, ctx.flags).unwrap();
  });

  let out = String::from_utf8(output).unwrap();
  assert!(out.contains("Found git"));
  assert!(out.contains("Found cmake"));
  assert!(out.contains("Found meson"));
}
