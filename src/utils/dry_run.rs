use crate::ctx::RunCtx;
use std::path::Path;

pub fn dry_run_or_do(
  verb: &str,
  progressive: &str,
  path: &Path,
  ctx: &mut RunCtx,
  timer_label: &str,
  op: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
  if ctx.flags.dry_run {
    writeln!(ctx.io.output, "  Would {verb}: {}", path.display()).ok();
  } else {
    writeln!(ctx.io.output, "  {progressive}: {}", path.display()).ok();
    crate::time!(ctx.flags.timing, ctx.io.output, timer_label, { op() })?;
    writeln!(ctx.io.output, "  Done").ok();
  }
  Ok(())
}
