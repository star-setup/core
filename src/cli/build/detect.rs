use crate::{cli::BuildSystem, ctx::RunCtx, prompts::ask_choice};
use std::path::{Path, PathBuf};

fn pick_build_system(
  has_cmake: bool,
  has_meson: bool,
  none_err: &str,
  ctx: &mut RunCtx<'_>,
) -> Result<BuildSystem, String> {
  match (has_cmake, has_meson) {
    (true, false) => Ok(BuildSystem::Cmake),
    (false, true) => Ok(BuildSystem::Meson),
    (true, true) => match ask_choice(
      "Multiple build systems detected:",
      &["CMake", "Meson"],
      &mut ctx.io,
    )? {
      0 => Ok(BuildSystem::Cmake),
      1 => Ok(BuildSystem::Meson),
      _ => Err("Invalid build system choice".into()),
    },
    (false, false) => Err(none_err.into()),
  }
}

/// Detects the build system in use by inspecting the given directory.
/// # Errors
/// Returns an error on EOF during prompt, or if no supported build system is found.
pub fn detect_build_system(dir: &Path, ctx: &mut RunCtx<'_>) -> Result<BuildSystem, String> {
  crate::time!(ctx.io.timing, ctx.io.output, "Detect", {
    let has_cmake = dir.join("CMakeLists.txt").exists();
    let has_meson = dir.join("meson.build").exists();
    pick_build_system(has_cmake, has_meson, "No supported build system found", ctx)
  })
}

/// Detects the build system consistently across all repo directories.
/// # Errors
/// Returns an error if systems are inconsistent or none found, or EOF during prompt.
pub fn detect_mono_build_system(
  dirs: &[PathBuf],
  ctx: &mut RunCtx<'_>,
) -> Result<BuildSystem, String> {
  writeln!(ctx.io.output, "Detecting build system\n").ok();
  crate::time!(ctx.io.timing, ctx.io.output, "Detect", {
    let all_cmake = dirs.iter().all(|d| d.join("CMakeLists.txt").exists());
    let all_meson = dirs.iter().all(|d| d.join("meson.build").exists());
    pick_build_system(
      all_cmake,
      all_meson,
      "Repositories have inconsistent or missing build systems",
      ctx,
    )
  })
}
