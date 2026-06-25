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
