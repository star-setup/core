use crate::{ctx::IoCtx, repository::repo_dir_name};
use std::{borrow::Cow, collections::HashMap, path::Path};

/// Prints the setup completion summary including paths, executable location, and total timing.
pub fn print_setup_complete<S: std::hash::BuildHasher>(
  canonical_map: Option<&HashMap<String, String, S>>,
  mono_repo_path: &Path,
  build_path: &Path,
  test_repo: &str,
  total: std::time::Instant,
  io: &mut IoCtx<'_>,
) {
  writeln!(io.output, "Setup complete").ok();
  let mono_repo_disp =
    dunce::canonicalize(mono_repo_path).map_or_else(|_| Cow::Borrowed(mono_repo_path), Cow::Owned);
  writeln!(io.output, "Repositories in: {}", mono_repo_disp.display()).ok();

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
    let build_disp =
      dunce::canonicalize(build_path).map_or_else(|_| Cow::Borrowed(build_path), Cow::Owned);
    writeln!(io.output, "Build output in: {}", build_disp.display()).ok();
  }
  if io.timing {
    writeln!(io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
}
