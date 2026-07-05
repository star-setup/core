use crate::{
  build::BuildSystem,
  ctx::{IoCtx, RunCtx, RunFlags},
};
use std::path::Path;

/// Prints a "Would ..." summary in verbose dry-run mode, or a completed-action summary otherwise.
pub fn report_summary(io: &mut IoCtx<'_>, flags: RunFlags, would_msg: &str, done_msg: &str) {
  if flags.dry_run {
    if flags.verbose {
      writeln!(io.output, "  Would {would_msg}").ok();
    }
  } else {
    writeln!(io.output, "  {done_msg}").ok();
  }
}

/// Prints a dry-run message or executes `op`, timing it if not a dry run.
/// # Errors
/// Returns an error if `op` fails.
pub fn dry_run_or_do(
  verb: &str,
  progressive: &str,
  path: &Path,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
  timer_label: &str,
  op: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
  if flags.dry_run {
    if flags.verbose {
      writeln!(io.output, "  Would {verb}: {}", path.display()).ok();
    }
  } else {
    if flags.verbose {
      writeln!(io.output, "  {progressive}: {}", path.display()).ok();
    }
    crate::time!(flags.timing, io.output, timer_label, { op() })?;
    if flags.verbose {
      writeln!(io.output, "  Done").ok();
    }
  }
  Ok(())
}

/// Detects the build system or prints what would be detected in a dry run.
/// # Errors
/// Returns an error if `detect_fn` fails.
pub fn detect_or_dry_run<F>(
  bs_flag: Option<BuildSystem>,
  ctx: &mut RunCtx,
  detect_fn: F,
) -> Result<Option<BuildSystem>, String>
where
  F: FnOnce(&mut RunCtx) -> Result<BuildSystem, String>,
{
  writeln!(ctx.io.output, "Detecting build system").ok();
  let result = crate::time!(ctx.flags.timing, ctx.io.output, "Detect", {
    let r = if let Some(bs) = bs_flag {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Build system flag set: {bs:?}").ok();
      }
      Some(bs)
    } else if ctx.flags.dry_run {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Would detect build system after cloning").ok();
      }
      None
    } else {
      Some(detect_fn(ctx)?)
    };
    if ctx.flags.verbose {
      writeln!(ctx.io.output, "  Finished detecting").ok();
    }
    r
  });
  writeln!(ctx.io.output).ok();
  Ok(result)
}
