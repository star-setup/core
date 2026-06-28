use std::{
  collections::HashMap,
  io::Write,
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

/// Finds vcvars64.bat using vswhere.exe.
/// Returns None if vswhere is not found or no VS installation exists.
#[cfg(target_os = "windows")]
fn find_vcvars() -> Option<PathBuf> {
  let program_files =
    std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
  let vswhere = PathBuf::from(program_files).join(r"Microsoft Visual Studio\Installer\vswhere.exe");

  if !vswhere.exists() {
    return None;
  }

  let output = Command::new(&vswhere)
    .args(["-latest", "-property", "installationPath"])
    .output()
    .ok()?;

  let install_path = String::from_utf8(output.stdout).ok()?;
  let vcvars = PathBuf::from(install_path.trim()).join(r"VC\Auxiliary\Build\vcvars64.bat");

  vcvars.exists().then_some(vcvars)
}

/// Runs vcvars64.bat and captures the resulting environment variables.
/// # Errors
/// Returns an error if vcvars64.bat cannot be found or run.
#[cfg(target_os = "windows")]
fn get_msvc_env() -> Result<HashMap<String, String>, String> {
  let vcvars = find_vcvars().ok_or("Could not find vcvars64.bat via vswhere")?;
  let vcvars_str = vcvars.to_str().ok_or("Invalid vcvars path")?;

  let output = Command::new("cmd")
    .args(["/c", vcvars_str, "&&", "set"])
    .output()
    .map_err(|e| format!("Failed to run vcvars64.bat: {e}"))?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  Ok(
    stdout
      .lines()
      .filter_map(|line| {
        let (key, val) = line.split_once('=')?;
        Some((key.to_string(), val.to_string()))
      })
      .collect(),
  )
}

/// Runs a shell command with optional working directory.
/// Returns Result.
/// # Errors
/// Returns an error if the command is empty, fails to spawn, or exits with a non-zero status.
pub fn run_command(
  cmd: &[&str],
  cwd: Option<&Path>,
  verbose: bool,
  output: &mut (impl Write + ?Sized),
) -> Result<(), String> {
  let (exe, args) = match cmd {
    [] => return Err("No command provided".to_string()),
    [exe, args @ ..] => (exe, args),
  };

  if verbose {
    writeln!(output, "Running: {}", cmd.join(" ")).ok();
    if let Some(dir) = cwd {
      writeln!(output, "  in directory: {}", dir.display()).ok();
    }
  }

  let mut command = Command::new(exe);
  command.stdin(Stdio::null());
  command.args(args);

  if let Some(dir) = cwd {
    command.current_dir(dir);
  }

  if *exe == "git" {
    command.env("GIT_TERMINAL_PROMPT", "0");
    if std::env::var("GIT_SSH_COMMAND").is_err() {
      command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
  }

  #[cfg(target_os = "windows")]
  if std::env::var("VSINSTALLDIR").is_err()
    && Path::new(exe)
      .file_stem()
      .is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case("meson"))
  {
    if let Ok(env) = get_msvc_env() {
      command.envs(env);
    }
  }

  if verbose {
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status = command
      .status()
      .map_err(|e| format!("Failed to run command: {e}"))?;

    if !status.success() {
      return Err(format!("Command failed with exit code: {status}"));
    }
  } else {
    command.stdout(Stdio::null()).stderr(Stdio::piped());
    let child = command
      .spawn()
      .map_err(|e| format!("Failed to start command: {e}"))?;

    let execution_output = child
      .wait_with_output()
      .map_err(|e| format!("Failed to wait for command: {e}"))?;

    if !execution_output.status.success() {
      let msg = String::from_utf8_lossy(&execution_output.stderr);
      let msg = msg.trim();

      let mut err_msg = format!("Command failed with exit code: {}", execution_output.status);
      if !msg.is_empty() {
        err_msg = format!("{err_msg}\n{msg}");
      }
      return Err(err_msg);
    }
  }

  Ok(())
}
