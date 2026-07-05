use crate::{build::BuildSystem, ctx::RunCtx, prompts::ask_choice};
use std::path::{Path, PathBuf};

fn pick_build_system(
  detected: &[BuildSystem],
  none_err: &str,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<BuildSystem, String> {
  match detected {
    [] => Err(none_err.into()),
    [single] => Ok(*single),
    multiple => {
      let labels: Vec<&str> = multiple
        .iter()
        .map(|s| match s {
          BuildSystem::Cmake => "CMake",
          BuildSystem::Meson => "Meson",
          BuildSystem::Npm => "npm",
        })
        .collect();
      let choice = ask_choice("Multiple build systems detected:", &labels, &mut ctx.io)?;
      Ok(multiple[choice])
    }
  }
}

/// Detects the build system in use by inspecting the given directory.
/// # Errors
/// Returns an error on EOF during prompt, or if no supported build system is found.
pub fn detect_build_system(dir: &Path, ctx: &mut RunCtx<'_, '_>) -> Result<BuildSystem, String> {
  crate::time!(ctx.flags.timing, ctx.io.output, "Scanned directory", {
    let mut detected = Vec::new();
    if dir.join("CMakeLists.txt").exists() {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found CMakeLists.txt").ok();
      }
      detected.push(BuildSystem::Cmake);
    }
    if dir.join("meson.build").exists() {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found meson.build").ok();
      }
      detected.push(BuildSystem::Meson);
    }
    if dir.join("package.json").exists() {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found package.json").ok();
      }
      detected.push(BuildSystem::Npm);
    }
    pick_build_system(&detected, "No supported build system found", ctx)
  })
}

/// Detects the build system consistently across all repo directories.
/// # Errors
/// Returns an error if systems are inconsistent or none found, or EOF during prompt.
pub fn detect_mono_build_system(
  dirs: &[PathBuf],
  ctx: &mut RunCtx<'_, '_>,
) -> Result<BuildSystem, String> {
  crate::time!(ctx.flags.timing, ctx.io.output, "Scanned directories", {
    let mut detected = Vec::new();
    if dirs.iter().all(|d| d.join("CMakeLists.txt").exists()) {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found CMakeLists.txt").ok();
      }
      detected.push(BuildSystem::Cmake);
    }
    if dirs.iter().all(|d| d.join("meson.build").exists()) {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found meson.build").ok();
      }
      detected.push(BuildSystem::Meson);
    }
    if dirs.iter().all(|d| d.join("package.json").exists()) {
      if ctx.flags.verbose {
        writeln!(ctx.io.output, "  Found package.json").ok();
      }
      detected.push(BuildSystem::Npm);
    }
    pick_build_system(
      &detected,
      "Repositories have inconsistent or missing build systems",
      ctx,
    )
  })
}
