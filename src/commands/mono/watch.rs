use crate::{
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
};
use std::{fs, path::Path};

/// Reads a lib's package.json and returns the appropriate watch command.
fn get_watch_command(
  repos_path: &Path,
  dir: &str,
  io: &mut IoCtx<'_>,
  flags: &RunFlags,
) -> Option<String> {
  let pkg_path = repos_path.join(dir).join("package.json");
  match fs::read_to_string(&pkg_path) {
    Err(_) => {
      if flags.verbose {
        writeln!(
          io.output,
          "  Warning: could not read {dir}/package.json, skipping"
        )
        .ok();
      }
      None
    }
    Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
      Err(_) => {
        if flags.verbose {
          writeln!(
            io.output,
            "  Warning: malformed {dir}/package.json, skipping"
          )
          .ok();
        }
        None
      }
      Ok(json) => {
        let scripts = json.get("scripts")?;
        if scripts.get("watch").is_some() {
          Some(format!("npm --workspace=repos/{dir} run watch"))
        } else if scripts.get("build").is_some() {
          Some(format!("npm --workspace=repos/{dir} run build -- --watch"))
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
    },
  }
}

/// Generates watch scripts for npm mono-repo mode.
/// # Errors
/// Returns an error if the scripts cannot be written.
pub fn generate_watch_scripts(
  mono_dir: &Path,
  repos_path: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: &RunFlags,
) -> Result<(), String> {
  let lib_dirs: Vec<String> = repos.iter().skip(1).map(|r| repo_dir_name(r)).collect();

  if lib_dirs.is_empty() {
    return Ok(());
  }

  let ps1_lines: Vec<String> = lib_dirs
    .iter()
    .filter_map(|d| {
      get_watch_command(repos_path, d, io, flags).map(|cmd| {
        format!(
          "Start-Process powershell -ArgumentList '-NoExit', '-Command', 'cd \"{}\"; {cmd}'",
          mono_dir.display()
        )
      })
    })
    .collect();

  let sh_lines: Vec<String> = lib_dirs
    .iter()
    .filter_map(|d| {
      get_watch_command(repos_path, d, io, flags)
        .map(|cmd| format!("cd \"{}\" && {cmd} &", mono_dir.display()))
    })
    .collect();

  let ps1_content = format!("# Watch all lib repositories\n{}\n", ps1_lines.join("\n"));
  let sh_content = format!(
    "#!/bin/bash\ntrap 'kill $(jobs -p)' EXIT\n# Watch all lib repositories\n{}\nwait\n",
    sh_lines.join("\n")
  );

  fs::write(mono_dir.join("watch.ps1"), ps1_content)
    .map_err(|e| format!("Failed to write watch.ps1: {e}"))?;
  fs::write(mono_dir.join("watch.sh"), sh_content)
    .map_err(|e| format!("Failed to write watch.sh: {e}"))?;

  writeln!(
    io.output,
    "Generated watch scripts at {}",
    mono_dir.display()
  )
  .ok();

  if flags.verbose {
    writeln!(io.output, "Watching {} libraries:", lib_dirs.len()).ok();
    for d in &lib_dirs {
      let full_path =
        dunce::canonicalize(repos_path.join(d)).unwrap_or_else(|_| repos_path.join(d));
      writeln!(io.output, "  {d:<24} -> {}", full_path.display()).ok();
    }
  }

  Ok(())
}

/// Opens watch scripts in new terminals.
/// # Errors
/// Returns an error if the terminal cannot be opened.
pub fn open_watch_scripts(mono_dir: &Path, io: &mut IoCtx<'_>) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    let ps1_path = mono_dir.join("watch.ps1");
    std::process::Command::new("powershell")
      .args([
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        ps1_path.to_str().ok_or("Invalid path")?,
      ])
      .spawn()
      .map_err(|e| format!("Failed to open watch.ps1: {e}"))?;
  }

  #[cfg(not(target_os = "windows"))]
  {
    let sh_path = mono_dir.join("watch.sh");
    std::process::Command::new("bash")
      .arg(sh_path.to_str().ok_or("Invalid path")?)
      .spawn()
      .map_err(|e| format!("Failed to open watch.sh: {e}"))?;
  }

  writeln!(io.output, "Opening watch scripts").ok();
  Ok(())
}
