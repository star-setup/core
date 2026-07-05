use crate::{
  build::read_package_json,
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
  utils::{dry_run_or_do, report_summary},
};
use std::{fs::write, path::Path};

/// Generates a root `package.json` wiring all repositories as npm workspaces.
/// # Errors
/// Returns an error if the `package.json` file cannot be written to `mono_dir`.
pub fn create_mono_repo_package_json(
  mono_dir: &Path,
  repos_path: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  writeln!(io.output, "  Creating npm workspace configuration").ok();

  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();

  let workspaces = module_names
    .iter()
    .map(|m| format!("    \"repos/{m}\""))
    .collect::<Vec<_>>()
    .join(",\n");

  // Read package name from each lib repo (skip first — test repo)
  let mut overrides = Vec::new();
  for (i, dir) in module_names.iter().enumerate() {
    if i == 0 {
      continue;
    }
    if let Some(json) = read_package_json(&repos_path.join(dir), "skipping override", io, flags) {
      if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
        overrides.push(format!("    \"{name}\": \"*\""));
      }
    }
  }

  let overrides_block = if overrides.is_empty() {
    String::new()
  } else {
    format!(",\n  \"overrides\": {{\n{}\n  }}", overrides.join(",\n"))
  };

  let content = format!(
    "{{\n  \"name\": \"star-setup-workspace\",\n  \"private\": true,\n  \"workspaces\": [\n{workspaces}\n  ]{overrides_block}\n}}\n"
  );

  let file_path = mono_dir.join("package.json");
  dry_run_or_do(
    "create file",
    "Creating",
    &file_path,
    io,
    flags,
    "Generate package.json",
    || write(&file_path, content).map_err(|e| e.to_string()),
  )?;

  report_summary(
    io,
    flags,
    &format!("create root package.json at {}\n", mono_dir.display()),
    &format!("Created root package.json at {}\n", mono_dir.display()),
  );

  Ok(())
}
