use crate::config::{ConfigEntry, SetupConfig};
use std::{fmt::Write as FmtWrite, io::Write};

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
  writeln!(out, "  Timing flag: {}", e.timing).ok();
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

/// Lists all saved configuration entries.
pub fn list_configs(config: &SetupConfig, output: &mut impl Write) {
  if config.configs.is_empty() {
    writeln!(output, "  No configurations created.").ok();
    writeln!(
      output,
      "  Run with --init-config to create a default configuration."
    )
    .ok();
    return;
  }

  writeln!(output, "Configurations:").ok();
  for (name, e) in &config.configs {
    writeln!(output, "\n{name}:").ok();
    write!(output, "{}", format_entry(e)).ok();
  }
}
