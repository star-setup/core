use std::collections::HashMap;
use crate::{ctx::IoCtx, repository::repo_dir_name};

/// Prints the setup completion summary including paths, executable location, and total timing.
pub fn print_setup_complete<S: std::hash::BuildHasher>(
  canonical_map: Option<HashMap<String, String, S>>,
  mono_repo_path: std::path::PathBuf,
  build_path: std::path::PathBuf,
  test_repo: &str,
  total: std::time::Instant,
  io: &mut IoCtx<'_>,
) {
  writeln!(io.output, "Setup complete").ok();
  writeln!(
    io.output,
    "Repositories in: {}",
    dunce::canonicalize(&mono_repo_path)
      .unwrap_or(mono_repo_path)
      .display()
  )
  .ok();
  if let Some(map) = canonical_map {
    let test_repo_name = repo_dir_name(test_repo);
    if let Some((canonical, _)) = map.iter().find(|(_, v)| *v == &test_repo_name) {
      let exe_name = if cfg!(windows) {
        format!("{canonical}.exe")
      } else {
        canonical.clone()
      };
      let exe_path = build_path
        .join("repos")
        .join(&test_repo_name)
        .join(&exe_name);
      writeln!(
        io.output,
        "Executable: {}",
        dunce::canonicalize(&exe_path).unwrap_or(exe_path).display()
      )
      .ok();
    }
  } else {
    writeln!(
      io.output,
      "Build output in: {}",
      dunce::canonicalize(&build_path)
        .unwrap_or(build_path)
        .display()
    )
    .ok();
  }
  if io.timing {
    writeln!(io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
}
