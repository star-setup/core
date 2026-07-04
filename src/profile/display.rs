use crate::{config::Config, ctx::IoCtx, profile::Profile};
use std::io::Write;

pub fn print_profile_details(
  output: &mut (impl Write + ?Sized),
  title: &str,
  label: &str,
  profile: &Profile,
) {
  let Profile { test_repo, deps } = profile;
  writeln!(output, "  {title}").ok();
  if let Some(t) = test_repo {
    writeln!(output, "  {t}").ok();
  }
  if !deps.is_empty() {
    writeln!(output, "    {label}: {}", deps.len()).ok();
    for d in deps {
      writeln!(output, "      - {d}").ok();
    }
  }
}

/// Lists all configured profiles.
pub fn list_profiles(config: &Config, io: &mut IoCtx<'_>) {
  if config.profiles.is_empty() {
    writeln!(
      io.output,
      "No profiles configured. Run with profile add to create a new profile."
    )
    .ok();
    return;
  }
  writeln!(io.output, "Configured profiles:\n").ok();
  for (name, profile) in &config.profiles {
    print_profile_details(io.output, name, "Repositories", profile);
    writeln!(io.output).ok();
  }
}
