use crate::{cli::build::BuildSystem, prompts::ask_choice};
use std::{
  io::{BufRead, Write},
  path::{Path, PathBuf},
};

/// Detects the build system in use by inspecting the given directory.
/// # Errors
/// Returns an error on EOF during prompt, or if no supported build system is found.
pub fn detect_build_system(
  dir: &Path,
  input: &mut impl BufRead,
  output: &mut impl Write,
  timing: bool,
) -> Result<BuildSystem, String> {
  let t = std::time::Instant::now();
  let has_cmake = dir.join("CMakeLists.txt").exists();
  let has_meson = dir.join("meson.build").exists();
  let result = match (has_cmake, has_meson) {
    (true, false) => Ok(BuildSystem::Cmake),
    (false, true) => Ok(BuildSystem::Meson),
    (true, true) => match ask_choice(
      "Multiple build systems detected:",
      &["CMake", "Meson"],
      input,
      output,
    )? {
      0 => Ok(BuildSystem::Cmake),
      _ => Ok(BuildSystem::Meson),
    },
    (false, false) => Err("No supported build system found".into()),
  };
  if timing {
    writeln!(output, "  [timing] Detect: {:.2?}", t.elapsed()).ok();
  }
  result
}

/// Detects the build system consistently across all repo directories.
/// # Errors
/// Returns an error if systems are inconsistent or none found, or EOF during prompt.
pub fn detect_mono_build_system(
  dirs: &[PathBuf],
  input: &mut impl BufRead,
  output: &mut impl Write,
  timing: bool,
) -> Result<BuildSystem, String> {
  writeln!(output, "Detecting build system\n").ok();
  let t = std::time::Instant::now();
  let all_cmake = dirs.iter().all(|d| d.join("CMakeLists.txt").exists());
  let all_meson = dirs.iter().all(|d| d.join("meson.build").exists());
  let result = match (all_cmake, all_meson) {
    (true, false) => Ok(BuildSystem::Cmake),
    (false, true) => Ok(BuildSystem::Meson),
    (true, true) => match ask_choice(
      "Multiple build systems detected:",
      &["CMake", "Meson"],
      input,
      output,
    )? {
      0 => Ok(BuildSystem::Cmake),
      _ => Ok(BuildSystem::Meson),
    },
    (false, false) => Err("Repositories have inconsistent or missing build systems".into()),
  };
  if timing {
    writeln!(output, "  [timing] Detect: {:.2?}", t.elapsed()).ok();
  }
  result
}
