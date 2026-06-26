//! Mode header rendering

use crate::ctx::IoCtx;

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
pub fn print_mode_header(header: &ModeHeader<'_>, io: &mut IoCtx<'_>) {
  writeln!(io.output, "Star Setup: {}", header.mode).ok();
  if let Some(p) = header.profile {
    writeln!(io.output, "  Profile: {p}").ok();
  }
  if let Some(r) = header.test_repo {
    writeln!(io.output, "  Test Repository: {r}").ok();
  } else if let Some(r) = header.repo_name {
    writeln!(io.output, "  Repository: {r}").ok();
  }
  writeln!(
    io.output,
    "  Clone Method: {}",
    if header.use_ssh { "SSH" } else { "HTTPS" }
  )
  .ok();
  if let Some(d) = header.mono_dir {
    writeln!(io.output, "  Directory: {d}").ok();
  }
  if let Some(c) = header.lib_count {
    writeln!(io.output, "  Libraries: {c}").ok();
  }
  writeln!(io.output).ok();
}
