//! Build system dispatch and per-system build functions.

use crate::cli::ResolvedArgs;
use crate::utils::run_command;
use std::io::Write;
use std::path::Path;

/// Runs `CMake` configuration and optionally builds the project in `build_path`.
/// # Errors
/// Returns an error if any `CMake` command fails.
pub fn cmake_build(
  args: &ResolvedArgs,
  build_path: &Path,
  mono: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  let build_type_flag = format!("-DCMAKE_BUILD_TYPE={}", args.build.build_type.to_cmake());
  let mut cmake_cmd = if mono {
    vec!["cmake", "-DBUILD_LOCAL=ON", &build_type_flag, ".."]
  } else {
    vec!["cmake", "..", &build_type_flag]
  };
  cmake_cmd.extend(args.build.cmake_flags.iter().map(String::as_str));
  run_command(
    &cmake_cmd,
    Some(build_path),
    args.connection.verbose,
    output,
  )?;
  if !args.build.no_build {
    writeln!(output, "Building project\n").ok();
    run_command(
      &[
        "cmake",
        "--build",
        ".",
        "--config",
        args.build.build_type.to_cmake(),
      ],
      Some(build_path),
      args.connection.verbose,
      output,
    )?;
  }
  Ok(())
}
