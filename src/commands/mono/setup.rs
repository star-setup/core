use crate::{
  cli::BuildSystem,
  commands::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild, hoist_wraps},
  ctx::RunCtx,
  repository::repo_dir_name,
};
use std::{fs, path::PathBuf};

/// Generates root build configuration files for the mono-repo.
/// # Errors
/// Returns an error if config file generation or wrap hoisting fails.
pub fn generate_mono_config(
  build_system: &BuildSystem,
  mono_repo_path: &std::path::Path,
  repos_path: &std::path::Path,
  repo_dirs: &[PathBuf],
  repos: &[String],
  ctx: &mut RunCtx<'_>,
) -> Result<Option<std::collections::HashMap<String, String>>, String> {
  writeln!(ctx.io.output, "Creating mono-repo configuration").ok();
  match build_system {
    BuildSystem::Cmake => {
      create_mono_repo_cmakelists(mono_repo_path, repos, &mut ctx.io)?;
      Ok(None)
    }
    BuildSystem::Meson => {
      let map = hoist_wraps(repos_path, repo_dirs, &mut ctx.io)?;
      let subproject_names: Vec<String> = repos
        .iter()
        .map(|r| {
          let dir = repo_dir_name(r);
          map
            .iter()
            .find(|(_, v)| *v == &dir)
            .map(|(k, _)| k.clone())
            .unwrap_or(dir)
        })
        .collect();
      create_mono_repo_mesonbuild(mono_repo_path, &subproject_names, &mut ctx.io)?;
      Ok(Some(map))
    }
  }
}

/// Prepares the build directory, optionally cleaning it first.
/// # Errors
/// Returns an error if the build directory cannot be created or removed.
pub fn prepare_build_dir(
  build_path: &std::path::Path,
  clean: bool,
  ctx: &mut RunCtx<'_>,
) -> Result<(), String> {
  if clean && build_path.exists() {
    writeln!(ctx.io.output, "Cleaning build directory\n").ok();
    crate::time!(ctx.io.timing, ctx.io.output, "Clean", {
      fs::remove_dir_all(build_path).map_err(|e| e.to_string())?;
    });
  }

  writeln!(ctx.io.output, "Creating build directory\n").ok();
  crate::time!(ctx.io.timing, ctx.io.output, "Create build directory", {
    fs::create_dir_all(build_path).map_err(|e| e.to_string())?;
  });
  Ok(())
}

/// Builds the full ordered list of repositories, deduplicating by directory name.
#[must_use]
pub fn build_repo_list(test_repo: &str, deps: &[String]) -> Vec<String> {
  let mut seen = std::collections::HashSet::new();
  std::iter::once(test_repo.to_string())
    .chain(deps.iter().cloned())
    .filter(|r| seen.insert(repo_dir_name(r)))
    .collect()
}
