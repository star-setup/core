use crate::{
  build::{build_project, detect_build_system, maybe_open_dev_server, BuildSystem::Npm},
  commands::{extract_repo_input, prepare_build_dir, print_mode_header, ModeHeader},
  ctx::RunCtx,
  prompts::confirm,
  repository::{
    clone_repo, repo_dir_name,
    ExistsAction::{Skip, Update},
  },
  resolve::ResolvedArgs,
  utils::detect_or_dry_run,
};
use std::{path::Path, time::Instant};

/// Clones and configures a single repository.
/// # Errors
/// Returns an error if no repository is specified, or if any git or build system fails.
pub fn single_repo_mode(
  args: &ResolvedArgs,
  base_dir: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let total = Instant::now();
  let repo = extract_repo_input(args)?;
  let dir_name = repo_dir_name(repo);
  let repo_path = base_dir.join(&dir_name);

  print_mode_header(
    &ModeHeader {
      mode: "Single Repository Mode",
      test_repo: None,
      repo_name: Some(&dir_name),
      use_ssh: args.connection.ssh,
      mono_dir: None,
      profile: None,
      lib_count: None,
      repo_count: None,
    },
    &mut ctx.io,
  );

  writeln!(ctx.io.output, "Cloning repository").ok();
  clone_repo(
    repo,
    base_dir,
    args.connection.ssh,
    |io| {
      Ok(if confirm("  Update existing repository?", args.yes, io)? {
        Update
      } else {
        Skip
      })
    },
    ctx,
  )?;
  writeln!(ctx.io.output).ok();

  let build_path = repo_path.join(&args.build.build_dir);
  let build_system = detect_or_dry_run(args.build.build_system, ctx, |ctx| {
    detect_build_system(&repo_path, ctx)
  })?;

  if let Some(build_system) = build_system {
    if build_system == Npm {
      if args.build.clean && ctx.flags.verbose {
        writeln!(ctx.io.output, "  --clean has no effect for npm projects").ok();
      }
      build_project(args, &repo_path, &repo_path, build_system, false, ctx)?;
    } else {
      prepare_build_dir(&build_path, args.build.clean, ctx)?;
      build_project(args, &build_path, &repo_path, build_system, false, ctx)?;
    }
  }

  if ctx.flags.dry_run || build_system.is_none() {
    writeln!(ctx.io.output, "Would finish in {dir_name}").ok();
  } else if build_system == Some(Npm) {
    writeln!(ctx.io.output, "Project finished in {dir_name}").ok();
  } else {
    writeln!(
      ctx.io.output,
      "Project finished in {dir_name}/{}",
      args.build.build_dir
    )
    .ok();
  }

  if ctx.flags.timing {
    writeln!(ctx.io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }

  maybe_open_dev_server(args, build_system, &repo_path, ctx)
}
