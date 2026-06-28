use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildSystem {
  /// `CMake` build system (`CMakeLists.txt`).
  Cmake,
  /// Meson build system (`meson.build`).
  Meson,
}

impl FromStr for BuildSystem {
  type Err = String;

  /// Parses a build system string.
  /// # Errors
  /// Returns an error if the string does not match `cmake` or `meson`.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let systems = [("cmake", Self::Cmake), ("meson", Self::Meson)];

    for (name, variant) in systems {
      if s.eq_ignore_ascii_case(name) {
        return Ok(variant);
      }
    }

    Err(format!("Unknown build system '{s}'. Valid: cmake, meson"))
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildType {
  /// Debug build with no optimizations.
  #[default]
  Debug,
  /// Optimized release build.
  Release,
  /// Release build with debug info.
  #[serde(alias = "relwithdebinfo", alias = "debugoptimized")]
  RelWithDebInfo,
  /// Minimized binary size release build.
  #[serde(alias = "minsizerel", alias = "minsize")]
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

impl FromStr for BuildType {
  type Err = String;

  /// Parses a build type string, accepting canonical and system-specific aliases.
  /// # Errors
  /// Returns an error if the string does not match any known build type.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let types: &[(&[&str], Self)] = &[
      (&["debug"], Self::Debug),
      (&["release"], Self::Release),
      (
        &["rel-with-deb-info", "relwithdebinfo", "debugoptimized"],
        Self::RelWithDebInfo,
      ),
      (&["min-size-rel", "minsizerel", "minsize"], Self::MinSizeRel),
    ];

    for (aliases, variant) in types {
      for alias in *aliases {
        if s.eq_ignore_ascii_case(alias) {
          return Ok(*variant);
        }
      }
    }

    Err(format!(
      "Unknown build type '{s}'. Canonical: debug, release, rel-with-deb-info, min-size-rel"
    ))
  }
}
