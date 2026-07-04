use crate::{
  cli::{BuildSystem, ResolvedArgs},
  commands::read_package_json,
  ctx::RunCtx,
};
use std::path::Path;

/// Runs `cmd`, timing it if `ctx.flags.timing` is set.
/// # Errors
/// Returns an error if the command fails.
fn run_timed(
  cmd: &[&str],
  cwd: Option<&Path>,
  label: &str,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  crate::time!(ctx.flags.timing, ctx.io.output, label, {
    ctx.runner.run(cmd, cwd, ctx.flags, ctx.io.output)?;
  });
  Ok(())
}

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

/// Detects and dispatches to the appropriate build system.
/// # Errors
/// Returns an error if detection or the build system command fails.
pub fn build_project(
  args: &ResolvedArgs,
  build_path: &Path,
  source_path: &Path,
  build_system: BuildSystem,
  mono: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let result = match build_system {
    BuildSystem::Cmake => cmake_build(args, build_path, mono, ctx),
    BuildSystem::Meson => meson_build(args, build_path, source_path, ctx),
    BuildSystem::Npm => npm_build(args, source_path, mono, ctx),
  };
  writeln!(ctx.io.output).ok();
  result
}
