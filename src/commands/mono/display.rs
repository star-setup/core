use crate::{
  build::{BuildSystem, BuildSystem::Npm},
  commands::mono::SetupPaths,
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
};
use dunce::canonicalize;
use std::{collections::HashMap, hash::BuildHasher, path::Path, time::Instant};

/// Resolves display paths for setup completion summary.
#[must_use]
pub fn resolve_setup_paths<S: BuildHasher>(
  canonical_map: Option<&HashMap<String, String, S>>,
  mono_repo_path: &Path,
  build_path: &Path,
  test_repos: &[String],
  build_system: Option<BuildSystem>,
) -> SetupPaths {
  let mono_repo_disp =
    canonicalize(mono_repo_path).unwrap_or_else(|_| mono_repo_path.to_path_buf());

  let (exe_paths, build_disp) = if let Some(map) = canonical_map {
    let exe_paths = test_repos
      .iter()
      .filter_map(|test_repo| {
        let test_repo_name = repo_dir_name(test_repo);
        map
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
            (test_repo_name.clone(), canonicalize(&p).unwrap_or(p))
          })
      })
      .collect();
    (exe_paths, None)
  } else {
    let build_disp = if build_system == Some(Npm) {
      None
    } else {
      Some(canonicalize(build_path).unwrap_or_else(|_| build_path.to_path_buf()))
    };
    (Vec::new(), build_disp)
  };

  SetupPaths {
    mono_repo_disp,
    exe_paths,
    build_disp,
  }
}

/// Prints the setup completion summary.
pub fn print_setup_complete(
  paths: &SetupPaths,
  total: Instant,
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) {
  writeln!(io.output, "  Setup complete").ok();
  writeln!(
    io.output,
    "  Repositories in: {}",
    paths.mono_repo_disp.display()
  )
  .ok();
  if !paths.exe_paths.is_empty() {
    if flags.verbose {
      for (name, exe) in &paths.exe_paths {
        writeln!(io.output, "  Executable [{name}]: {}", exe.display()).ok();
      }
    } else {
      writeln!(io.output, "  Executables: {}", paths.exe_paths.len()).ok();
    }
  }
  if let Some(build) = &paths.build_disp {
    writeln!(io.output, "  Build output in: {}", build.display()).ok();
  }
  if flags.timing {
    writeln!(io.output, "  [timing] Total: {:.2?}", total.elapsed()).ok();
  }
}
