use crate::{
  ctx::{IoCtx, RunCtx, RunFlags},
  repository::repo_dir_name,
  utils::{dry_run_or_do, report_summary},
};
use std::{fs::write, path::Path};

/// Runs `cmd`, timing it if `ctx.flags.timing` is set.
/// # Errors
/// Returns an error if the command fails.
pub fn run_timed(
  cmd: &[&str],
  cwd: Option<&Path>,
  label: &str,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  crate::time!(ctx.flags.timing, ctx.io.output, label, {
    ctx.runner.run(cmd, cwd, ctx.flags, ctx.io.output)?;
  });
  Ok(())
}

/// Shared helper to generate, write, and log monorepo build configuration files.
/// # Errors
/// Returns an error if the config file cannot be written to `mono_dir`.
pub fn write_mono_repo_config(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
  filename: &str,
  format_modules: impl Fn(&[String]) -> String,
  render_template: impl Fn(&str) -> String,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_str = format_modules(&module_names);
  let content = render_template(&modules_str);
  let file_path = mono_dir.join(filename);

  dry_run_or_do(
    "create file",
    "Creating",
    &file_path,
    io,
    flags,
    &format!("Generate {filename}"),
    || write(&file_path, content).map_err(|e| e.to_string()),
  )?;

  report_summary(
    io,
    flags,
    &format!("create root {filename} at {}\n", mono_dir.display()),
    &format!("Created root {filename} at {}\n", mono_dir.display()),
  );

  Ok(())
}
