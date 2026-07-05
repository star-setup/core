use crate::{build::common::run_timed, ctx::RunCtx, resolve::ResolvedArgs};
use std::path::Path;

fn to_str(path: &Path) -> Result<&str, String> {
  path
    .to_str()
    .ok_or_else(|| format!("Invalid path: {}", path.display()))
}

/// Runs Meson configuration and optionally builds the project in `build_path`.
/// # Errors
/// Returns an error if any Meson command fails.
pub fn meson_build(
  args: &ResolvedArgs,
  build_path: &Path,
  source_path: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let buildtype_flag = format!("--buildtype={}", args.build.build_type.to_meson());
  let mut meson_cmd = vec!["meson", "setup"];
  meson_cmd.push(&buildtype_flag);
  meson_cmd.push(to_str(build_path)?);
  meson_cmd.push(to_str(source_path)?);
  meson_cmd.extend(args.build.meson_flags.iter().map(String::as_str));

  run_timed(&meson_cmd, None, "Meson setup", ctx)?;
  if !args.build.no_build {
    writeln!(ctx.io.output, "Building project").ok();
    run_timed(
      &["meson", "compile", "-C", to_str(build_path)?],
      None,
      "Meson compile",
      ctx,
    )?;
  }
  Ok(())
}
