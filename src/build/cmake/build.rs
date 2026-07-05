use crate::{build::common::run_timed, ctx::RunCtx, resolve::ResolvedArgs};
use std::path::Path;

/// Runs `CMake` configuration and optionally builds the project in `build_path`.
/// # Errors
/// Returns an error if any `CMake` command fails.
pub fn cmake_build(
  args: &ResolvedArgs,
  build_path: &Path,
  mono: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let build_type_flag = format!("-DCMAKE_BUILD_TYPE={}", args.build.build_type.to_cmake());
  let mut cmake_cmd = if mono {
    vec!["cmake", "-DBUILD_LOCAL=ON", &build_type_flag, ".."]
  } else {
    vec!["cmake", "..", &build_type_flag]
  };
  cmake_cmd.extend(args.build.cmake_flags.iter().map(String::as_str));

  run_timed(&cmake_cmd, Some(build_path), "CMake configure", ctx)?;

  if !args.build.no_build {
    writeln!(ctx.io.output, "Building project").ok();
    run_timed(
      &[
        "cmake",
        "--build",
        ".",
        "--config",
        args.build.build_type.to_cmake(),
      ],
      Some(build_path),
      "CMake build",
      ctx,
    )?;
  }
  Ok(())
}
