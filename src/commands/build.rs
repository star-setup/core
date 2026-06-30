//! Build system dispatch and per-system build functions.

use crate::{
  cli::{BuildSystem, ResolvedArgs},
  ctx::RunCtx,
};
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

  crate::time!(ctx.flags.timing, ctx.io.output, "CMake configure", {
    ctx
      .runner
      .run(&cmake_cmd, Some(build_path), ctx.flags, ctx.io.output)?;
  });

  if !args.build.no_build {
    writeln!(ctx.io.output, "Building project\n").ok();
    crate::time!(ctx.flags.timing, ctx.io.output, "CMake build", {
      ctx.runner.run(
        &[
          "cmake",
          "--build",
          ".",
          "--config",
          args.build.build_type.to_cmake(),
        ],
        Some(build_path),
        ctx.flags,
        ctx.io.output,
      )?;
    });
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

  crate::time!(ctx.flags.timing, ctx.io.output, "Meson setup", {
    ctx
      .runner
      .run(&meson_cmd, None, ctx.flags, ctx.io.output)?;
  });
  if !args.build.no_build {
    writeln!(ctx.io.output, "Building project\n").ok();
    crate::time!(ctx.flags.timing, ctx.io.output, "Meson compile", {
      ctx.runner.run(
        &["meson", "compile", "-C", to_str(build_path)?],
        None,
        ctx.flags,
        ctx.io.output,
      )?;
    });
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
  crate::time!(ctx.flags.timing, ctx.io.output, "npm install", {
    ctx.runner.run(
      &["npm", "install"],
      Some(source_path),
      ctx.flags,
      ctx.io.output,
    )?;
  });
  if !args.build.no_build && !is_mono {
    writeln!(ctx.io.output, "Building project\n").ok();
    crate::time!(ctx.flags.timing, ctx.io.output, "npm build", {
      ctx.runner.run(
        &["npm", "run", "build"],
        Some(source_path),
        ctx.flags,
        ctx.io.output,
      )?;
    });
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
  match build_system {
    BuildSystem::Cmake => cmake_build(args, build_path, mono, ctx),
    BuildSystem::Meson => meson_build(args, build_path, source_path, ctx),
    BuildSystem::Npm => npm_build(args, source_path, mono, ctx),
  }
}
