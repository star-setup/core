use crate::config::SetupConfig;
use std::{fs, io, io::Write, path::PathBuf};

/// Loads configuration from the first valid JSON file in `locations`.
pub fn load_config(locations: &[PathBuf], output: &mut impl Write) -> SetupConfig {
  let mut invalid_count = 0;

  for path in locations {
    if !path.exists() {
      continue;
    }
    match fs::read_to_string(path) {
      Ok(contents) => match serde_json::from_str::<SetupConfig>(&contents) {
        Ok(mut config) => {
          config.path = Some(path.clone());
          return config;
        }
        Err(e) => {
          writeln!(output, "Warning: Invalid JSON in {}: {e}", path.display()).ok();
          invalid_count += 1;
        }
      },
      Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
        writeln!(output, "Error: No permission to read {}", path.display()).ok();
        invalid_count += 1;
      }
      Err(e) => {
        writeln!(
          output,
          "An unexpected error occurred reading {}: {e}",
          path.display()
        )
        .ok();
        invalid_count += 1;
      }
    }
  }

  if invalid_count != 0 {
    writeln!(
      output,
      "Found {invalid_count} config file{} that had errors",
      if invalid_count == 1 { "" } else { "s" }
    )
    .ok();
  }
  SetupConfig::new()
}

/// Serializes the configuration and writes it to the path stored in `config.path`.
/// # Errors
/// Returns an error if serialization fails or if the file cannot be written.
pub fn save_config(config: &mut SetupConfig) -> Result<PathBuf, String> {
  let path = config
    .path
    .get_or_insert_with(|| {
      dirs::home_dir().map_or_else(
        || PathBuf::from(".star-setup.json"),
        |h| h.join(".star-setup.json"),
      )
    })
    .clone();
  let json =
    serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {e}"))?;

  fs::write(&path, json).map_err(|e| match e.kind() {
    io::ErrorKind::PermissionDenied => {
      format!("Error: No permission to write to {}", path.display())
    }
    _ => format!(
      "An unexpected error occurred writing {}: {}",
      path.display(),
      e
    ),
  })?;
  Ok(path)
}
