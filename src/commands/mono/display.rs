use crate::{
  cli::BuildSystem,
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
};
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

/// Resolved display paths for the setup completion summary.
pub struct SetupPaths {
  /// Canonicalized path to the mono-repo root directory.
  pub mono_repo_disp: PathBuf,
  /// Canonicalized path to the test repository executable, if found.
  pub exe_path: Option<PathBuf>,
  /// Canonicalized path to the build output directory, if no canonical map was provided.
  pub build_disp: Option<PathBuf>,
}

/// Resolves display paths for setup completion summary.
#[must_use]
pub fn resolve_setup_paths<S: std::hash::BuildHasher>(
  canonical_map: Option<&HashMap<String, String, S>>,
  mono_repo_path: &Path,
  build_path: &Path,
  test_repo: &str,
  build_system: Option<BuildSystem>,
) -> SetupPaths {
  let mono_repo_disp =
    dunce::canonicalize(mono_repo_path).unwrap_or_else(|_| mono_repo_path.to_path_buf());

  let (exe_path, build_disp) = if let Some(map) = canonical_map {
    let test_repo_name = repo_dir_name(test_repo);
    let exe_path = map
      .iter()
      .find(|(_, v)| *v == &test_repo_name)
      .map(|(canonical, _)| {
        let exe_name = if cfg!(windows) {
          format!("{canonical}.exe")
        } else {
          canonical.clone()
        };
        let p = build_path
          .join("repos")
          .join(&test_repo_name)
          .join(&exe_name);
        dunce::canonicalize(&p).unwrap_or(p)
      });
    (exe_path, None)
  } else {
    let build_disp = if build_system == Some(BuildSystem::Npm) {
      None
    } else {
      Some(dunce::canonicalize(build_path).unwrap_or_else(|_| build_path.to_path_buf()))
    };
    (None, build_disp)
  };

  SetupPaths {
    mono_repo_disp,
    exe_path,
    build_disp,
  }
}

/// Prints the setup completion summary.
pub fn print_setup_complete(
  paths: &SetupPaths,
  total: std::time::Instant,
  io: &mut IoCtx<'_>,
  flags: &RunFlags,
) {
  writeln!(io.output, "Setup complete").ok();
  writeln!(
    io.output,
    "Repositories in: {}",
    paths.mono_repo_disp.display()
  )
  .ok();
  if let Some(exe) = &paths.exe_path {
    writeln!(io.output, "Executable: {}", exe.display()).ok();
  }
  if let Some(build) = &paths.build_disp {
    writeln!(io.output, "Build output in: {}", build.display()).ok();
  }
  if flags.timing {
    writeln!(io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
}
