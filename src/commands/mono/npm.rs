use crate::ctx::{IoCtx, RunFlags};
use serde_json::{from_str, Value};
use std::{fs::read_to_string, path::Path};

/// Reads and parses `{repos_path}/{dir}/package.json`, warning (if verbose) and
/// returning `None` on I/O or parse failure.
pub fn read_package_json(
  repos_path: &Path,
  dir: &str,
  skip_suffix: &str,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Option<Value> {
  let pkg_path = repos_path.join(dir).join("package.json");
  match read_to_string(&pkg_path) {
    Err(_) => {
      if flags.verbose {
        writeln!(
          io.output,
          "  Warning: could not read {dir}/package.json, {skip_suffix}"
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
            "  Warning: malformed {dir}/package.json, {skip_suffix}"
          )
          .ok();
        }
        None
      }
      Ok(json) => Some(json),
    },
  }
}
