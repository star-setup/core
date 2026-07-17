use crate::{
  ctx::{IoCtx, RunFlags},
  utils::{dry_run_or_do, report_summary},
};
use std::{
  fs::write,
  path::{Path, PathBuf},
  process::Command,
};

/// Writes `<name>.ps1` / `<name>.sh` launching each `(dir, cmd)` entry in its own
/// terminal, so the set can be reopened later by rerunning the script.
/// # Errors
/// Returns an error if the scripts cannot be written.
pub fn generate_terminal_scripts(
  name: &str,
  header: &str,
  mono_dir: &Path,
  entries: &[(PathBuf, String)],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  let ps1_lines: Vec<String> = entries
    .iter()
    .map(|(dir, cmd)| {
      format!(
        "Start-Process powershell -ArgumentList '-NoExit', '-Command', 'cd \"{}\"; {cmd}'",
        dir.display()
      )
    })
    .collect();
  let sh_lines: Vec<String> = entries
    .iter()
    .map(|(dir, cmd)| format!("cd \"{}\" && {cmd} &", dir.display()))
    .collect();

  let ps1_content = format!("# {header}\n{}\n", ps1_lines.join("\n"));
  let sh_content = format!(
    "#!/bin/bash\ntrap 'kill $(jobs -p)' EXIT\n# {header}\n{}\nwait\n",
    sh_lines.join("\n")
  );

  let ps1_name = format!("{name}.ps1");
  let sh_name = format!("{name}.sh");
  dry_run_or_do(
    &format!("write {name} scripts"),
    "Writing",
    mono_dir,
    io,
    flags,
    "Write scripts",
    || {
      write(mono_dir.join(&ps1_name), ps1_content)
        .map_err(|e| format!("Failed to write {ps1_name}: {e}"))?;
      write(mono_dir.join(&sh_name), sh_content)
        .map_err(|e| format!("Failed to write {sh_name}: {e}"))?;
      Ok(())
    },
  )?;

  report_summary(
    io,
    flags,
    &format!("generate {name} scripts at {}", mono_dir.display()),
    &format!("Generated {name} scripts at {}", mono_dir.display()),
  );
  Ok(())
}

/// Opens the generated `<name>` scripts in new terminals.
/// # Errors
/// Returns an error if the terminal cannot be opened.
pub fn open_scripts(
  name: &str,
  mono_dir: &Path,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  if flags.dry_run {
    if flags.verbose {
      writeln!(io.output, "  Would open {name} scripts").ok();
    }
    return Ok(());
  }

  crate::time!(flags.timing, io.output, "Open", {
    #[cfg(target_os = "windows")]
    {
      let ps1_path = mono_dir.join(format!("{name}.ps1"));
      Command::new("powershell")
        .args([
          "-ExecutionPolicy",
          "Bypass",
          "-File",
          ps1_path.to_str().ok_or("Invalid path")?,
        ])
        .spawn()
        .map_err(|e| format!("Failed to open {name}.ps1: {e}"))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
      let sh_path = mono_dir.join(format!("{name}.sh"));
      Command::new("bash")
        .arg(sh_path.to_str().ok_or("Invalid path")?)
        .spawn()
        .map_err(|e| format!("Failed to open {name}.sh: {e}"))?;
    }
    Ok::<(), String>(())
  })?;
  writeln!(io.output, "  Opening {name} scripts").ok();
  Ok(())
}
