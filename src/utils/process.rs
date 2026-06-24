use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

/// Finds vcvars64.bat using vswhere.exe.
/// Returns None if vswhere is not found or no VS installation exists.
#[cfg(target_os = "windows")]
fn find_vcvars() -> Option<std::path::PathBuf> {
  let vswhere = std::path::PathBuf::from(
    std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string()),
  )
  .join(r"Microsoft Visual Studio\Installer\vswhere.exe");

  if !vswhere.exists() {
    return None;
  }

  let output = Command::new(&vswhere)
    .args(["-latest", "-property", "installationPath"])
    .output()
    .ok()?;

  let install_path = String::from_utf8(output.stdout).ok()?;
  let vcvars =
    std::path::PathBuf::from(install_path.trim()).join(r"VC\Auxiliary\Build\vcvars64.bat");

  vcvars.exists().then_some(vcvars)
}

/// Runs vcvars64.bat and captures the resulting environment variables.
/// # Errors
/// Returns an error if vcvars64.bat cannot be found or run.
#[cfg(target_os = "windows")]
fn get_msvc_env() -> Result<std::collections::HashMap<String, String>, String> {
  let vcvars = find_vcvars().ok_or("Could not find vcvars64.bat via vswhere")?;
  let output = Command::new("cmd")
    .args([
      "/c",
      vcvars.to_str().ok_or("Invalid vcvars path")?,
      "&&",
      "set",
    ])
    .output()
    .map_err(|e| format!("Failed to run vcvars64.bat: {e}"))?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  Ok(
    stdout
      .lines()
      .filter_map(|line| {
        let mut parts = line.splitn(2, '=');
        Some((parts.next()?.to_string(), parts.next()?.to_string()))
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
  output: &mut impl Write,
) -> Result<(), String> {
  if cmd.is_empty() {
    return Err("No command provided".to_string());
  }

  if verbose {
    writeln!(output, "Running: {}", cmd.join(" ")).ok();
    if let Some(dir) = cwd {
      writeln!(output, "  in directory: {}", dir.display()).ok();
    }
  }

  let mut command = Command::new(cmd[0]);
  command.stdin(Stdio::null());
  if cmd[0] == "git" {
    command.env("GIT_TERMINAL_PROMPT", "0");
    if std::env::var("GIT_SSH_COMMAND").is_err() {
      command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
  }
  #[cfg(target_os = "windows")]
  if cmd[0] == "meson" && std::env::var("VSINSTALLDIR").is_err() {
    if let Ok(env) = get_msvc_env() {
      for (k, v) in env {
        command.env(k, v);
      }
    }
  }

  if verbose {
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
  } else {
    command.stdout(Stdio::null());
    command.stderr(Stdio::piped());
  }

  command.args(&cmd[1..]);
  if let Some(dir) = cwd {
    command.current_dir(dir);
  }

  if verbose {
    let status = command
      .status()
      .map_err(|e| format!("Failed to run command: {e}"))?;
    if status.success() {
      return Ok(());
    }
    return Err(format!("Command failed with exit code: {status}"));
  }

  let mut child = command
    .spawn()
    .map_err(|e| format!("Failed to start command: {e}"))?;
  let stderr_handle = child.stderr.take();
  let stderr_thread = thread::spawn(move || {
    let mut s = String::new();
    if let Some(mut h) = stderr_handle {
      h.read_to_string(&mut s).ok();
    }
    s
  });

  let status = child
    .wait()
    .map_err(|e| format!("Failed to wait for command: {e}"))?;
  let stderr = stderr_thread.join().unwrap_or_default();
  if status.success() {
    Ok(())
  } else {
    let msg: &str = stderr.trim();
    if msg.is_empty() {
      Err(format!("Command failed with exit code: {status}"))
    } else {
      Err(format!("Command failed with exit code: {status}\n{msg}"))
    }
  }
}
