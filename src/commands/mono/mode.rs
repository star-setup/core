use crate::{
  cli::{detect_mono_build_system, BuildSystem, ResolvedArgs},
  commands::{
    build_repo_list, configure_and_build, extract_repo_input,
    mono::{
      clone_mono_repos,
      display::{resolve_setup_paths, SetupPaths},
      generate_mono_config, generate_watch_scripts, open_watch_scripts, print_setup_complete,
    },
    prepare_build_dir, resolve_repos_for_mono, resolve_test_repo,
  },
  config::SetupConfig,
  ctx::RunCtx,
  repository::repo_dir_name,
};
use std::{
  fs,
  path::{Path, PathBuf},
};

/// Clones and configures a mono-repo ecosystem from a profile or explicit repository list.
/// # Errors
/// Returns an error if no repository is specified, directory creation fails, or any build system command fails.
pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &SetupConfig,
  base_dir: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let total = std::time::Instant::now();

  let repo_input = extract_repo_input(args)?;
  let test_repo = resolve_test_repo(repo_input)?;
  let deps = resolve_repos_for_mono(args, config, &test_repo, &mut ctx.io)?;
  let repos = build_repo_list(&test_repo, &deps);
  writeln!(ctx.io.output, "Total repositories: {}\n", repos.len()).ok();

  let mono_repo_path = base_dir.join(&args.mono.mono_dir);
  let repos_path = mono_repo_path.join("repos");
  if ctx.flags.dry_run {
    writeln!(
      ctx.io.output,
      "Would create directory: {}",
      repos_path.display()
    )
    .ok();
  } else {
    crate::time!(ctx.flags.timing, ctx.io.output, "Create directory", {
      fs::create_dir_all(&repos_path).map_err(|e| e.to_string())?;
    });
  }

  clone_mono_repos(&repos, &repos_path, args.connection.ssh, ctx)?;

  let repo_dirs: Vec<PathBuf> = repos
    .iter()
    .map(|r| repos_path.join(repo_dir_name(r)))
    .collect();

  let build_path = mono_repo_path.join(&args.build.build_dir);

  let build_system = if let Some(bs) = args.build.build_system {
    Some(bs)
  } else if !ctx.flags.dry_run {
    Some(detect_mono_build_system(&repo_dirs, ctx)?)
  } else {
    None
  };

  let canonical_map = if let Some(bs) = build_system {
    let map = generate_mono_config(bs, &mono_repo_path, &repos_path, &repo_dirs, &repos, ctx)?;
    if bs != BuildSystem::Npm {
      prepare_build_dir(build_path.as_path(), args.build.clean, ctx)?;
    }
    configure_and_build(args, &mono_repo_path, &build_path, bs, true, ctx)?;
    map
  } else {
    prepare_build_dir(build_path.as_path(), args.build.clean, ctx)?;
    None
  };

  if build_system == Some(BuildSystem::Npm) && !args.build.no_watch && !ctx.flags.dry_run {
    generate_watch_scripts(&mono_repo_path, &repos_path, &repos, &mut ctx.io, ctx.flags)?;
    if args.build.watch {
      open_watch_scripts(&mono_repo_path, &mut ctx.io)?;
    }
  }

  let paths = if ctx.flags.dry_run {
    SetupPaths {
      mono_repo_disp: mono_repo_path.clone(),
      exe_path: None,
      build_disp: if build_system == Some(BuildSystem::Npm) {
        None
      } else {
        Some(build_path.clone())
      },
    }
  } else {
    resolve_setup_paths(
      canonical_map.as_ref(),
      &mono_repo_path,
      &build_path,
      &test_repo,
      build_system,
    )
  };

  print_setup_complete(&paths, total, &mut ctx.io, ctx.flags);
  Ok(())
}
