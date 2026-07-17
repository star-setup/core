use crate::{
  build::{generate_terminal_scripts, read_package_json},
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
};
use dunce::canonicalize;
use std::path::{Path, PathBuf};

/// Reads a lib's package.json and returns the appropriate watch command.
fn get_watch_command(
  repos_path: &Path,
  dir: &str,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Option<String> {
  let json = read_package_json(&repos_path.join(dir), "skipping", io, flags)?;
  let scripts = json.get("scripts")?;
  if scripts.get("watch").is_some() {
    Some(format!("npm --workspace=repos/{dir} run watch"))
  } else if scripts.get("build").is_some() {
    Some(format!(
      "npm --workspace=repos/{dir} run build ''--'' --watch"
    ))
  } else {
    if flags.verbose {
      writeln!(
        io.output,
        "  Warning: {dir} has no watch or build script, skipping"
      )
      .ok();
    }
    None
  }
}

/// Generates watch scripts for npm mono-repo mode.
/// # Errors
/// Returns an error if the scripts cannot be written.
pub fn generate_watch_scripts(
  mono_dir: &Path,
  repos_path: &Path,
  deps: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<bool, String> {
  let lib_dirs: Vec<String> = deps.iter().map(|r| repo_dir_name(r)).collect();
  if lib_dirs.is_empty() {
    return Ok(false);
  }

  let entries: Vec<(PathBuf, String)> = lib_dirs
    .iter()
    .filter_map(|d| {
      get_watch_command(repos_path, d, io, flags).map(|cmd| (mono_dir.to_path_buf(), cmd))
    })
    .collect();

  generate_terminal_scripts(
    "watch",
    "Watch all lib repositories",
    mono_dir,
    &entries,
    io,
    flags,
  )?;

  if flags.verbose {
    writeln!(io.output, "  Watching {} libraries:", lib_dirs.len()).ok();
    for d in &lib_dirs {
      let full_path = canonicalize(repos_path.join(d)).unwrap_or_else(|_| repos_path.join(d));
      writeln!(io.output, "  {d:<24} -> {}", full_path.display()).ok();
    }
  }

  Ok(true)
}
