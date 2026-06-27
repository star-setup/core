use crate::{cli::BuildSystem, ctx::RunCtx, prompts::ask_choice};
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

pub fn detect_build_system(dir: &Path, ctx: &mut RunCtx<'_, '_>) -> Result<BuildSystem, String> {
  crate::time!(ctx.flags.timing, ctx.io.output, "Detect", {
    let mut detected = Vec::new();
    if dir.join("CMakeLists.txt").exists() {
      detected.push(BuildSystem::Cmake);
    }
    if dir.join("meson.build").exists() {
      detected.push(BuildSystem::Meson);
    }
    if dir.join("package.json").exists() {
      detected.push(BuildSystem::Npm);
    }
    pick_build_system(&detected, "No supported build system found", ctx)
  })
}

pub fn detect_mono_build_system(
  dirs: &[PathBuf],
  ctx: &mut RunCtx<'_, '_>,
) -> Result<BuildSystem, String> {
  writeln!(ctx.io.output, "Detecting build system\n").ok();
  crate::time!(ctx.flags.timing, ctx.io.output, "Detect", {
    let mut detected = Vec::new();
    if dirs.iter().all(|d| d.join("CMakeLists.txt").exists()) {
      detected.push(BuildSystem::Cmake);
    }
    if dirs.iter().all(|d| d.join("meson.build").exists()) {
      detected.push(BuildSystem::Meson);
    }
    if dirs.iter().all(|d| d.join("package.json").exists()) {
      detected.push(BuildSystem::Npm);
    }
    pick_build_system(
      &detected,
      "Repositories have inconsistent or missing build systems",
      ctx,
    )
  })
}
