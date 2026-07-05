use crate::{
  build::{common::run_timed, read_package_json},
  ctx::RunCtx,
  resolve::ResolvedArgs,
};
use std::path::Path;

/// Runs `npm install` and optionally builds the project in `source_path`.
/// # Errors
/// Returns an error if any npm command fails.
pub fn npm_build(
  args: &ResolvedArgs,
  source_path: &Path,
  is_mono: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  writeln!(ctx.io.output, "Installing dependencies").ok();
  run_timed(&["npm", "install"], Some(source_path), "npm install", ctx)?;
  if !args.build.no_build && !is_mono {
    let has_build = read_package_json(source_path, "skipping build", &mut ctx.io, ctx.flags)
      .is_some_and(|j| j.get("scripts").and_then(|s| s.get("build")).is_some());
    if has_build {
      writeln!(ctx.io.output, "Building project").ok();
      run_timed(
        &["npm", "run", "build"],
        Some(source_path),
        "npm build",
        ctx,
      )?;
    } else if ctx.flags.verbose {
      writeln!(ctx.io.output, "  No build script found, skipping build").ok();
    }
  }
  Ok(())
}
