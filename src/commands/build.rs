//! Build system dispatch and per-system build functions.

use crate::{
  cli::build::{detect_build_system, BuildSystem},
  cli::ResolvedArgs,
  utils::process::run_command,
};
use std::{
  io::{BufRead, Write},
  path::Path,
};

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

/// Runs Meson configuration and optionally builds the project in `build_path`.
/// # Errors
/// Returns an error if any Meson command fails.
pub fn meson_build(
  args: &ResolvedArgs,
  build_path: &Path,
  source_path: &Path,
  output: &mut impl Write,
) -> Result<(), String> {
  let buildtype_flag = format!("--buildtype={}", args.build.build_type.to_meson());
  let mut meson_cmd = vec!["meson", "setup"];
  meson_cmd.push(&buildtype_flag);
  meson_cmd.push(build_path.to_str().ok_or("Invalid build path")?);
  meson_cmd.push(source_path.to_str().ok_or("Invalid source path")?);
  meson_cmd.extend(args.build.meson_flags.iter().map(String::as_str));
  run_command(&meson_cmd, None, args.connection.verbose, output)?;
  if !args.build.no_build {
    writeln!(output, "Building project\n").ok();
    run_command(
      &[
        "meson",
        "compile",
        "-C",
        build_path.to_str().ok_or("Invalid build path")?,
      ],
      None,
      args.connection.verbose,
      output,
    )?;
  }
  Ok(())
}

/// Detects and dispatches to the appropriate build system.
/// # Errors
/// Returns an error if detection or the build system command fails.
pub fn build_project(
  args: &ResolvedArgs,
  build_path: &Path,
  source_path: &Path,
  mono: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  match detect_build_system(source_path, input, output)? {
    BuildSystem::Cmake => cmake_build(args, build_path, mono, output),
    BuildSystem::Meson => meson_build(args, build_path, source_path, output),
  }
}
