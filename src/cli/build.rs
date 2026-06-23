use serde::{Deserialize, Serialize};
use std::path::Path;

pub enum BuildSystem {
  /// `CMake` build system (`CMakeLists.txt`).
  Cmake,
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
pub fn detect_build_system(dir: &Path) -> Result<BuildSystem, String> {
  if dir.join("CMakeLists.txt").exists() {
    Ok(BuildSystem::Cmake)
  } else {
    Err("No supported build system found".into())
  }
}
