use crate::prompts::ask_choice;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

pub enum BuildSystem {
  /// `CMake` build system (`CMakeLists.txt`).
  Cmake,
  /// Meson build system (`meson.build`).
  Meson,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BuildType {
  /// Debug build with no optimizations.
  #[default]
  Debug,
  /// Optimized release build.
  Release,
  /// Release build with debug info.
  RelWithDebInfo,
  /// Minimized binary size release build.
  MinSizeRel,
}

impl BuildType {
  #[must_use]
  pub fn to_cmake(&self) -> &'static str {
    match self {
      Self::Debug => "Debug",
      Self::Release => "Release",
      Self::RelWithDebInfo => "RelWithDebInfo",
      Self::MinSizeRel => "MinSizeRel",
    }
  }

  #[must_use]
  pub fn to_meson(&self) -> &'static str {
    match self {
      Self::Debug => "debug",
      Self::Release => "release",
      Self::RelWithDebInfo => "debugoptimized",
      Self::MinSizeRel => "minsize",
    }
  }
}

impl std::str::FromStr for BuildType {
  type Err = String;

  /// Parses a build type string, accepting canonical and system-specific aliases.
  /// # Errors
  /// Returns an error if the string does not match any known build type.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "debug" => Ok(Self::Debug),
      "release" => Ok(Self::Release),
      "rel-with-deb-info" | "relwithdebinfo" | "debugoptimized" => Ok(Self::RelWithDebInfo),
      "min-size-rel" | "minsizerel" | "minsize" => Ok(Self::MinSizeRel),
      _ => Err(format!(
        "Unknown build type '{s}'. Canonical: debug, release, rel-with-deb-info, min-size-rel"
      )),
    }
  }
}

/// Detects the build system in use by inspecting the given directory.
/// # Errors
/// Returns an error on EOF during prompt, or if no supported build system is found.
pub fn detect_build_system(
  dir: &Path,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<BuildSystem, String> {
  let has_cmake = dir.join("CMakeLists.txt").exists();
  let has_meson = dir.join("meson.build").exists();
  match (has_cmake, has_meson) {
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
  }
}

/// Detects the build system consistently across all repo directories.
/// # Errors
/// Returns an error if systems are inconsistent or none found, or EOF during prompt.
pub fn detect_mono_build_system(
  dirs: &[PathBuf],
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<BuildSystem, String> {
  let all_cmake = dirs.iter().all(|d| d.join("CMakeLists.txt").exists());
  let all_meson = dirs.iter().all(|d| d.join("meson.build").exists());
  match (all_cmake, all_meson) {
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
  }
}
