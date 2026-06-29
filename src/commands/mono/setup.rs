use crate::{
  cli::BuildSystem,
  commands::{create_mono_repo_cmakelists, create_mono_repo_mesonbuild, hoist_wraps},
  ctx::RunCtx,
  repository::repo_dir_name,
};
use std::path::PathBuf;

/// Generates root build configuration files for the mono-repo.
/// # Errors
/// Returns an error if config file generation or wrap hoisting fails.
pub fn generate_mono_config(
  build_system: BuildSystem,
  mono_repo_path: &std::path::Path,
  repos_path: &std::path::Path,
  repo_dirs: &[PathBuf],
  repos: &[String],
  ctx: &mut RunCtx<'_, '_>,
) -> Result<Option<std::collections::HashMap<String, String>>, String> {
  writeln!(ctx.io.output, "Creating mono-repo configuration").ok();
  match build_system {
    BuildSystem::Cmake => {
      create_mono_repo_cmakelists(mono_repo_path, repos, &mut ctx.io, &mut ctx.flags)?;
      Ok(None)
    }
    BuildSystem::Meson => {
      let map = hoist_wraps(repos_path, repo_dirs, &mut ctx.io, &mut ctx.flags)?;
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
      create_mono_repo_mesonbuild(
        mono_repo_path,
        &subproject_names,
        &mut ctx.io,
        &mut ctx.flags,
      )?;
      Ok(Some(map))
    }
  }
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
