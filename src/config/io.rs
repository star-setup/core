use crate::{
  config::SetupConfig,
  ctx::{IoCtx, RunFlags},
};
use serde_json::{from_str, to_string_pretty};
use std::{
  fs::{read_to_string, write},
  io::{Error, ErrorKind::PermissionDenied, Write},
  path::{Path, PathBuf},
};

/// Returns the list of paths to search for a config file.
#[must_use]
pub fn config_locations(path: &Path) -> Vec<PathBuf> {
  [
    Some(path.to_path_buf()),
    dirs::home_dir().map(|h| h.join(path)),
  ]
  .into_iter()
  .flatten()
  .collect()
}

fn io_error_msg(verb: &str, path: &Path, e: &Error) -> String {
  match e.kind() {
    PermissionDenied => format!("Error: No permission to {verb} {}", path.display()),
    _ => format!(
      "An unexpected error occurred: {verb} {}: {e}",
      path.display()
    ),
  }
}

/// Loads configuration from the first valid JSON file in `locations`.
pub fn load_config(
  locations: &[PathBuf],
  verbose: bool,
  timing: bool,
  output: &mut impl Write,
) -> SetupConfig {
  if verbose {
    writeln!(output, "Loading config").ok();
  }

  let mut invalid_count = 0;

  for path in locations {
    if !path.exists() {
      if verbose {
        writeln!(output, "  Config path not found: {}", path.display()).ok();
      }
      continue;
    }
    let result = crate::time!(timing, output, "Read config", {
      match read_to_string(path) {
        Ok(contents) => match from_str::<SetupConfig>(&contents) {
          Ok(mut config) => {
            config.path = Some(path.clone());
            if verbose {
              writeln!(output, "  Loaded config from {}", path.display()).ok();
            }
            Some(config)
          }
          Err(e) => {
            writeln!(output, "  Warning: Invalid JSON in {}: {e}", path.display()).ok();
            invalid_count += 1;
            None
          }
        },
        Err(e) => {
          writeln!(output, "  {}", io_error_msg("read", path, &e)).ok();
          invalid_count += 1;
          None
        }
      }
    });
    if let Some(config) = result {
      if verbose || timing {
        writeln!(output).ok();
      }
      return config;
    }
  }

  if invalid_count != 0 {
    writeln!(
      output,
      "  Found {invalid_count} config file{} that had errors",
      if invalid_count == 1 { "" } else { "s" }
    )
    .ok();
  }
  if verbose || timing {
    writeln!(output).ok();
  }
  SetupConfig::new()
}

/// Serializes the configuration and writes it to the path stored in `config.path`.
/// # Errors
/// Returns an error if serialization fails or if the file cannot be written.
pub fn save_config(
  config: &mut SetupConfig,
  timing: bool,
  output: &mut impl Write,
) -> Result<PathBuf, String> {
  let path = config
    .path
    .get_or_insert_with(|| {
      dirs::home_dir().map_or_else(
        || PathBuf::from(".star-setup.json"),
        |h| h.join(".star-setup.json"),
      )
    })
    .clone();
  let json = to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {e}"))?;

  crate::time!(timing, output, "Write config", {
    write(&path, json).map_err(|e| io_error_msg("write to", &path, &e))?;
  });

  Ok(path)
}

/// Runs `mutate` and saves the config, or prints `would_msg` in dry-run mode
/// without mutating or saving. Calls `on_saved` with the saved path after a
/// successful save.
/// # Errors
/// Returns an error if saving the config file fails.
pub fn persist_or_dry_run(
  config: &mut SetupConfig,
  flags: RunFlags,
  io: &mut IoCtx<'_>,
  would_msg: &str,
  mutate: impl FnOnce(&mut SetupConfig),
  on_saved: impl FnOnce(&SetupConfig, &Path, &mut IoCtx<'_>),
) -> Result<(), String> {
  if flags.dry_run {
    writeln!(io.output, "  {would_msg}").ok();
  } else {
    mutate(config);
    let path = save_config(config, flags.timing, &mut io.output)?;
    on_saved(config, &path, io);
  }
  Ok(())
}
