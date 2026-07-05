use crate::ctx::{IoCtx, RunFlags};
use serde_json::{from_str, Value};
use std::{fs::read_to_string, path::Path};

/// Reads and parses `{repo_path}/package.json`, warning (if verbose) and
/// returning `None` on I/O or parse failure.
pub fn read_package_json(
  repo_path: &Path,
  skip_suffix: &str,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Option<Value> {
  let pkg_path = repo_path.join("package.json");
  match read_to_string(&pkg_path) {
    Err(_) => {
      if flags.verbose {
        writeln!(
          io.output,
          "  Warning: could not read {}, {skip_suffix}",
          pkg_path.display()
        )
        .ok();
      }
      None
    }
    Ok(content) => match from_str::<Value>(&content) {
      Err(_) => {
        if flags.verbose {
          writeln!(
            io.output,
            "  Warning: malformed {}, {skip_suffix}",
            pkg_path.display()
          )
          .ok();
        }
        None
      }
      Ok(json) => Some(json),
    },
  }
}
