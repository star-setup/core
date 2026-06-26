use crate::{
  cli::{detect_mono_build_system, ResolvedArgs},
  commands::{
    build_repo_list, configure_and_build, extract_repo_input,
    mono::{clone_mono_repos, generate_mono_config, print_setup_complete},
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
  ctx: &mut RunCtx<'_>,
) -> Result<(), String> {
  let total = std::time::Instant::now();

  let repo_input = extract_repo_input(args)?;
  let test_repo = resolve_test_repo(repo_input)?;
  let deps = resolve_repos_for_mono(args, config, &test_repo, &mut ctx.io)?;
  let repos = build_repo_list(&test_repo, &deps);
  writeln!(ctx.io.output, "Total repositories: {}\n", repos.len()).ok();

  let mono_repo_path = base_dir.join(&args.mono.mono_dir);
  let repos_path = mono_repo_path.join("repos");
  crate::time!(ctx.io.timing, ctx.io.output, "Create directory", {
    fs::create_dir_all(&repos_path).map_err(|e| e.to_string())?;
  });

  clone_mono_repos(&repos, &repos_path, args.connection.ssh, ctx)?;

  let repo_dirs: Vec<PathBuf> = repos
    .iter()
    .map(|r| repos_path.join(repo_dir_name(r)))
    .collect();

  let build_system = detect_mono_build_system(&repo_dirs, ctx)?;

  let canonical_map = generate_mono_config(
    &build_system,
    &mono_repo_path,
    &repos_path,
    &repo_dirs,
    &repos,
    ctx,
  )?;

  let build_path = mono_repo_path.join(&args.build.build_dir);
  prepare_build_dir(build_path.as_path(), args.build.clean, ctx)?;
  configure_and_build(args, &mono_repo_path, &build_path, build_system, true, ctx)?;

  print_setup_complete(
    canonical_map.as_ref(),
    &mono_repo_path,
    &build_path,
    &test_repo,
    total,
    &mut ctx.io,
  );
  Ok(())
}
