use crate::{
  build::{
    cmake_build, create_mono_repo_cmakelists, create_mono_repo_mesonbuild,
    create_mono_repo_package_json, hoist_wraps, meson_build, npm_build, BuildSystem,
    BuildSystem::Cmake, BuildSystem::Meson, BuildSystem::Npm,
  },
  cli::ResolvedArgs,
  ctx::RunCtx,
  repository::repo_dir_name,
};
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

/// Detects and dispatches to the appropriate build system.
/// # Errors
/// Returns an error if detection or the build system command fails.
pub fn build_project(
  args: &ResolvedArgs,
  build_path: &Path,
  source_path: &Path,
  build_system: BuildSystem,
  mono: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let result = match build_system {
    BuildSystem::Cmake => cmake_build(args, build_path, mono, ctx),
    BuildSystem::Meson => meson_build(args, build_path, source_path, ctx),
    BuildSystem::Npm => npm_build(args, source_path, mono, ctx),
  };
  writeln!(ctx.io.output).ok();
  result
}

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
) -> Result<Option<HashMap<String, String>>, String> {
  writeln!(ctx.io.output, "  Creating mono-repo configuration").ok();
  match build_system {
    Cmake => {
      create_mono_repo_cmakelists(mono_repo_path, repos, &mut ctx.io, ctx.flags)?;
      Ok(None)
    }
    Meson => {
      let map = hoist_wraps(repos_path, repo_dirs, &mut ctx.io, ctx.flags)?;
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
      create_mono_repo_mesonbuild(mono_repo_path, &subproject_names, &mut ctx.io, ctx.flags)?;
      Ok(Some(map))
    }
    Npm => {
      create_mono_repo_package_json(mono_repo_path, repos_path, repos, &mut ctx.io, ctx.flags)?;
      Ok(None)
    }
  }
}
