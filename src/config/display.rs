use crate::config::types::ConfigEntry;
use std::fmt::Write as FmtWrite;

/// Formats a `ConfigEntry` as a human-readable string.
#[must_use]
pub fn format_entry(e: &ConfigEntry) -> String {
  let mut out = String::new();
  writeln!(out, "  SSH: {}", e.ssh).ok();
  writeln!(out, "  Build Type: {}", e.build_type.to_cmake()).ok();
  writeln!(out, "  Build Directory: {}", e.build_dir).ok();
  writeln!(out, "  Mono-build Directory: {}", e.mono_dir).ok();
  writeln!(out, "  No-build flag: {}", e.no_build).ok();
  writeln!(out, "  Clean flag: {}", e.clean).ok();
  writeln!(out, "  Verbose flag: {}", e.verbose).ok();
  if e.cmake_flags.is_empty() {
    out.push('\n');
  } else if e.cmake_flags.len() == 1 {
    writeln!(out, "  CMake argument: {}", e.cmake_flags[0]).ok();
  } else {
    out.push_str("  CMake arguments:\n");
    for arg in &e.cmake_flags {
      writeln!(out, "    {arg}").ok();
    }
  }
  if e.meson_flags.is_empty() {
    out.push('\n');
  } else if e.meson_flags.len() == 1 {
    writeln!(out, "  Meson argument: {}", e.meson_flags[0]).ok();
  } else {
    out.push_str("  Meson arguments:\n");
    for arg in &e.meson_flags {
      writeln!(out, "    {arg}").ok();
    }
  }
  out
}
