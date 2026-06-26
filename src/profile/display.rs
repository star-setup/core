use crate::{config::types::SetupConfig, ctx::IoCtx};
use std::io::Write;

pub fn print_profile_details(
  output: &mut (impl Write + ?Sized),
  title: &str,
  label: &str,
  repos: &[String],
) {
  writeln!(output, "  {title}").ok();
  writeln!(output, "    {label}: {}", repos.len()).ok();
  for repo in repos {
    writeln!(output, "      - {repo}").ok();
  }
}

/// Lists all configured profiles.
pub fn list_profiles(config: &SetupConfig, io: &mut IoCtx<'_>) {
  if config.profiles.is_empty() {
    writeln!(
      io.output,
      "No profiles configured. Run with --init-config to create a default configuration."
    )
    .ok();
    return;
  }

  writeln!(io.output, "Configured profiles:\n").ok();
  for (name, repos) in &config.profiles {
    print_profile_details(io.output, name, "Repositories", repos);
    writeln!(io.output).ok();
  }
}
