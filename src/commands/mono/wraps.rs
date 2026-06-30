use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
};

use crate::ctx::{IoCtx, RunFlags};

/// Parses the `project()` name from `meson.build` content.
/// Returns the name with hyphens replaced by underscores, or `None` if not found.
#[must_use]
pub fn parse_project_name(content: &str) -> Option<String> {
  let needle = b"project(";
  let bytes = content.as_bytes();
  let mut i = 0;
  while i + needle.len() <= bytes.len() {
    if bytes[i..].starts_with(needle) {
      let preceding_ok = i == 0 || {
        let p = bytes[i - 1] as char;
        !p.is_alphanumeric() && p != '_'
      };
      if preceding_ok {
        let rest = &content[i + needle.len()..];
        let first_quote = rest.find(['"', '\''])?;
        let quote = rest.as_bytes()[first_quote] as char;
        let name_start = &rest[first_quote + 1..];
        let end = name_start.find(quote)?;
        return Some(name_start[..end].replace('-', "_"));
      }
    }
    i += 1;
  }
  None
}

/// Parses `[provide]` key-value pairs from wrap file content.
#[must_use]
pub fn parse_provide_pairs(content: &str) -> HashMap<String, String> {
  let mut in_provide = false;
  let mut pairs = HashMap::new();
  for line in content.lines() {
    let line = line.trim();
    if line.starts_with('[') {
      in_provide = line == "[provide]";
      continue;
    }
    if in_provide {
      if let Some((key, val)) = line.split_once('=') {
        let k = key.trim().to_string();
        let v = val.trim().to_string();
        if !k.is_empty() && !v.is_empty() {
          pairs.insert(k, v);
        }
      }
    }
  }
  pairs
}

/// Generates local-only wrap files in `repos_dir` bridging canonical dependency
/// names to owner-prefixed clone directories.
///
/// Reads each cloned repo's `project()` name as the join key, scans each repo's
/// `subprojects/*.wrap` for verbatim `[provide]` pairs, and emits
/// `repos/<canonical>.wrap` with `directory = <clone-dir>` and the provide line.
/// # Errors
/// Returns an error if any wrap file cannot be written or a directory cannot be read.
pub fn hoist_wraps(
  repos_dir: &Path,
  repo_dirs: &[PathBuf],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<HashMap<String, String>, String> {
  crate::time!(flags.timing, io.output, "Hoist wraps", {
    // normalized project name -> owner-prefixed dir name
    let mut project_to_dir: HashMap<String, String> = HashMap::new();
    for dir in repo_dirs {
      let meson_build = dir.join("meson.build");
      if !meson_build.exists() {
        continue;
      }
      let content = fs::read_to_string(&meson_build).map_err(|e| e.to_string())?;
      if let Some(name) = parse_project_name(&content) {
        if let Some(dir_name) = dir.file_name().and_then(|n| n.to_str()) {
          project_to_dir.insert(name, dir_name.to_string());
        }
      }
    }

    // collect [provide] pairs from each repo's subprojects/*.wrap
    let mut provides: HashMap<String, String> = HashMap::new();
    for dir in repo_dirs {
      let subprojects_dir = dir.join("subprojects");
      if !subprojects_dir.exists() {
        continue;
      }
      for entry in fs::read_dir(&subprojects_dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("wrap") {
          continue;
        }
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        for (key, val) in parse_provide_pairs(&content) {
          if project_to_dir.contains_key(&key) {
            provides.entry(key).or_insert(val);
          }
        }
      }
    }

    // emit hoisted wraps
    for (canonical_name, dir_name) in &project_to_dir {
      let wrap_content = if let Some(dep_var) = provides.get(canonical_name) {
        format!("[wrap-file]\ndirectory = {dir_name}\n\n[provide]\n{canonical_name} = {dep_var}\n")
      } else {
        format!("[wrap-file]\ndirectory = {dir_name}\n")
      };
      let wrap_path = repos_dir.join(format!("{canonical_name}.wrap"));
      fs::write(&wrap_path, &wrap_content).map_err(|e| e.to_string())?;
      writeln!(
        io.output,
        "  Generated wrap: {canonical_name}.wrap -> {dir_name}"
      )
      .ok();
    }

    Ok(project_to_dir)
  })
}
