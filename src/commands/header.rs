use crate::ctx::IoCtx;

/// Header information printed at the start of each command mode.
pub struct ModeHeader<'a> {
  pub mode: &'a str,
  pub test_repos: &'a [String],
  pub repo_name: Option<&'a str>,
  pub mono_dir: Option<&'a str>,
  pub profile: Option<&'a str>,
  pub lib_count: Option<usize>,
  pub repo_count: Option<usize>,
  pub use_ssh: bool,
  pub verbose: bool,
}

/// Prints a formatted header summarizing the current mode and configuration.
pub fn print_mode_header(header: &ModeHeader<'_>, io: &mut IoCtx<'_>) {
  writeln!(io.output, "Star Setup: {}", header.mode).ok();
  if let Some(p) = header.profile {
    writeln!(io.output, "  Profile: {p}").ok();
  }
  if !header.test_repos.is_empty() {
    if header.verbose {
      writeln!(io.output, "  Test Repositories:").ok();
      for r in header.test_repos {
        writeln!(io.output, "    {r}").ok();
      }
    } else {
      writeln!(
        io.output,
        "  Test Repositories: {}",
        header.test_repos.len()
      )
      .ok();
    }
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
  if let Some(c) = header.repo_count {
    writeln!(io.output, "  Total repositories: {c}").ok();
  }
  writeln!(io.output).ok();
}
