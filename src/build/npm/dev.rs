#[cfg(not(target_os = "windows"))]
use crate::utils::process::resolve_exe_args;
use crate::{
  build::{
    read_package_json,
    BuildSystem::{self, Npm},
  },
  ctx::{IoCtx, RunCtx, RunFlags},
  resolve::ResolvedArgs,
};
use serde_json::Value;
use std::{
  path::Path,
  process::{Command, Stdio},
};

/// Returns the dev command for a repo, or `None` if it has no `dev` script.
pub fn resolve_dev_command(
  repo_path: &Path,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Option<String> {
  let json = read_package_json(repo_path, "skipping dev server", io, flags)?;
  if json.get("scripts").and_then(|s| s.get("dev")).is_some() {
    return Some("npm run dev".to_string());
  }
  if is_vercel_project(repo_path, &json) {
    return Some("vercel dev".to_string());
  }
  if flags.verbose {
    writeln!(io.output, "  No dev script found, skipping dev server").ok();
  }
  None
}

fn is_vercel_project(repo_path: &Path, json: &Value) -> bool {
  repo_path.join("vercel.json").exists()
    || repo_path.join("api").is_dir()
    || ["dependencies", "devDependencies"]
      .iter()
      .any(|k| json.get(k).and_then(|d| d.get("@vercel/node")).is_some())
}

/// Opens the dev server in a new terminal.
/// # Errors
/// Returns an error if the terminal cannot be opened.
pub fn open_dev_server(
  repo_path: &Path,
  cmd: &str,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  if flags.dry_run {
    writeln!(io.output, "  Would start dev server: {cmd}").ok();
    return Ok(());
  }

  writeln!(io.output, "Starting dev server: {cmd}").ok();
  writeln!(
    io.output,
    "  in {}  (press Ctrl-C to stop)",
    repo_path.display()
  )
  .ok();

  #[cfg(target_os = "windows")]
  let mut command = {
    let cmd_esc = cmd.replace('\'', "''");
    let dir_esc = repo_path.display().to_string().replace('\'', "''");
    let mut c = Command::new("powershell");
    c.args([
      "-NoProfile",
      "-Command",
      "Start-Process",
      "powershell",
      "-ArgumentList",
      &format!("'-NoExit','-Command','{cmd_esc}'"),
      "-WorkingDirectory",
      &format!("'{dir_esc}'"),
    ]);
    c
  };
  #[cfg(not(target_os = "windows"))]
  let mut command = {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let resolved = resolve_exe_args(&parts);
    let mut c = Command::new(resolved[0]);
    c.args(&resolved[1..]);
    c.current_dir(repo_path);
    c
  };

  command
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null());

  command
    .spawn()
    .map(|_| ())
    .map_err(|e| format!("Failed to start dev server: {e}"))
}

/// Opens the project's dev server if `--dev` was passed and the build system is npm.
/// # Errors
/// Returns an error only if the dev server process fails to spawn.
pub fn maybe_open_dev_server(
  args: &ResolvedArgs,
  build_system: Option<BuildSystem>,
  repo_path: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  if !args.build.dev || build_system != Some(Npm) {
    return Ok(());
  }
  let cmd = if ctx.flags.dry_run {
    Some("npm run dev".to_string())
  } else {
    resolve_dev_command(repo_path, &mut ctx.io, ctx.flags)
  };
  if let Some(cmd) = cmd {
    open_dev_server(repo_path, &cmd, &mut ctx.io, ctx.flags)?;
  }
  Ok(())
}
