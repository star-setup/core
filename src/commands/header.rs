//! Mode header rendering
use std::io::Write;

/// Header information printed at the start of each command mode.
pub struct ModeHeader<'a> {
  pub mode: &'a str,
  pub test_repo: Option<&'a str>,
  pub repo_name: Option<&'a str>,
  pub use_ssh: bool,
  pub mono_dir: Option<&'a str>,
  pub profile: Option<&'a str>,
  pub lib_count: Option<usize>,
}

/// Prints a formatted header summarizing the current mode and configuration.
pub fn print_mode_header(header: &ModeHeader<'_>, output: &mut impl Write) {
  writeln!(output, "Star Setup: {}", header.mode).ok();
  if let Some(p) = header.profile {
    writeln!(output, "  Profile: {p}").ok();
  }
  if let Some(r) = header.test_repo {
    writeln!(output, "  Test Repository: {r}").ok();
  } else if let Some(r) = header.repo_name {
    writeln!(output, "  Repository: {r}").ok();
  }
  writeln!(
    output,
    "  Clone Method: {}",
    if header.use_ssh { "SSH" } else { "HTTPS" }
  )
  .ok();
  if let Some(d) = header.mono_dir {
    writeln!(output, "  Directory: {d}").ok();
  }
  if let Some(c) = header.lib_count {
    writeln!(output, "  Libraries: {c}").ok();
  }
  writeln!(output).ok();
}
