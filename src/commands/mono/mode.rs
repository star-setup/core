use crate::{
  build::{
    build_project, detect_mono_build_system, generate_dev_scripts, generate_mono_config,
    generate_watch_scripts, open_scripts, BuildSystem::Npm,
  },
  commands::{
    build_repo_list,
    mono::{
      print_setup_complete, resolve_profile, resolve_setup_paths, resolve_test_repos_for_mono,
      SetupPaths,
    },
    prepare_build_dir, print_mode_header, resolve_dep_repos_for_mono, ModeHeader,
  },
  config::Config,
  ctx::RunCtx,
  repository::{clone_repos, repo_dir_name},
  resolve::ResolvedArgs,
  utils::{detect_or_dry_run, dry_run_or_do},
};
use std::{
  fs,
  path::{Path, PathBuf},
  time::Instant,
};

/// Clones and configures a mono-repo ecosystem from a profile or explicit repository list.
/// # Errors
/// Returns an error if no repository is specified, directory creation fails, or any build system command fails.
pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &Config,
  base_dir: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let total = Instant::now();
  let profile = resolve_profile(args, config);
  let test_repos = resolve_test_repos_for_mono(args, profile)?;
  let deps = resolve_dep_repos_for_mono(args, profile);
  let repos = build_repo_list(&test_repos, &deps);

  print_mode_header(
    &ModeHeader {
      mode: if args.mono.profile.is_some() {
        "Profile"
      } else {
        "Mono-repository"
      },
      test_repos: &test_repos,
      repo_name: None,
      use_ssh: args.connection.ssh,
      mono_dir: Some(&args.mono.mono_dir),
      profile: args.mono.profile.as_deref(),
      lib_count: Some(deps.len()),
      repo_count: Some(repos.len()),
      verbose: ctx.flags.verbose,
    },
    &mut ctx.io,
  );

  let mono_repo_path = base_dir.join(&args.mono.mono_dir);
  let repos_path = mono_repo_path.join("repos");
  if ctx.flags.verbose {
    writeln!(ctx.io.output, "Creating directory").ok();
  }
  dry_run_or_do(
    "create directory",
    "Creating",
    &repos_path,
    &mut ctx.io,
    ctx.flags,
    "Create directory",
    || fs::create_dir_all(&repos_path).map_err(|e| e.to_string()),
  )?;
  if ctx.flags.verbose {
    writeln!(ctx.io.output).ok();
  }

  clone_repos(&repos, &repos_path, args.connection.ssh, args.yes, ctx)?;

  let repo_dirs: Vec<PathBuf> = repos
    .iter()
    .map(|r| repos_path.join(repo_dir_name(r)))
    .collect();

  let build_path = mono_repo_path.join(&args.build.build_dir);
  let build_system = detect_or_dry_run(args.build.build_system, ctx, |ctx| {
    detect_mono_build_system(&repo_dirs, ctx)
  })?;

  let canonical_map = if let Some(bs) = build_system {
    let map = generate_mono_config(bs, &mono_repo_path, &repos_path, &repo_dirs, &repos, ctx)?;
    if bs != Npm {
      prepare_build_dir(build_path.as_path(), args.build.clean, ctx)?;
    } else if args.build.clean && ctx.flags.verbose {
      writeln!(ctx.io.output, "  --clean has no effect for npm projects").ok();
    }
    build_project(args, &build_path, &mono_repo_path, bs, true, ctx)?;
    map
  } else {
    None
  };

  if build_system == Some(Npm)
    && !args.build.no_watch
    && generate_watch_scripts(&mono_repo_path, &repos_path, &deps, &mut ctx.io, ctx.flags)?
    && args.build.watch
  {
    open_scripts("watch", &mono_repo_path, &mut ctx.io, ctx.flags)?;
  }

  if build_system == Some(Npm)
    && !args.build.no_dev
    && generate_dev_scripts(
      &mono_repo_path,
      &repos_path,
      &test_repos,
      &mut ctx.io,
      ctx.flags,
    )?
    && args.build.dev
  {
    open_scripts("dev", &mono_repo_path, &mut ctx.io, ctx.flags)?;
  }

  let paths = if ctx.flags.dry_run {
    SetupPaths {
      mono_repo_disp: mono_repo_path.clone(),
      exe_paths: Vec::new(),
      build_disp: if build_system == Some(Npm) {
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
      &test_repos,
      build_system,
    )
  };

  if ctx.flags.dry_run || build_system.is_none() {
    writeln!(
      ctx.io.output,
      "Would finish setup in {}",
      paths.mono_repo_disp.display()
    )
    .ok();
  } else {
    print_setup_complete(&paths, total, &mut ctx.io, ctx.flags);
  }
  Ok(())
}
