use std::io::Write;

use crate::config::types::SetupConfig;

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
pub fn list_profiles(config: &SetupConfig, output: &mut (impl Write + ?Sized)) {
  if config.profiles.is_empty() {
    writeln!(
      output,
      "No profiles configured. Run with --init-config to create a default configuration."
    )
    .ok();
    return;
  }

  writeln!(output, "Configured profiles:\n").ok();
  for (name, repos) in &config.profiles {
    print_profile_details(output, name, "Repositories", repos);
    writeln!(output).ok();
  }
}
