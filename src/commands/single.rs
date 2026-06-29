use crate::{
  cli::{detect_build_system, ResolvedArgs},
  commands::{
    configure_and_build, extract_repo_input, prepare_build_dir, print_mode_header, ModeHeader,
  },
  ctx::RunCtx,
  prompts::confirm,
  repository::{clone_repository, pull_repository, repo_dir_name},
};
use std::path::Path;

/// Clones and configures a single repository.
/// # Errors
/// Returns an error if no repository is specified, or if any git or build system fails.
pub fn single_repo_mode(
  args: &ResolvedArgs,
  base_dir: &Path,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let total = std::time::Instant::now();

  let repo = extract_repo_input(args)?;
  let dir_name = repo_dir_name(repo);

  print_mode_header(
    &ModeHeader {
      mode: "Single Repository Mode",
      test_repo: None,
      repo_name: Some(&dir_name),
      use_ssh: args.connection.ssh,
      mono_dir: None,
      profile: None,
      lib_count: None,
    },
    &mut ctx.io,
  );

  let repo_path = base_dir.join(&dir_name);
  if repo_path.exists() {
    writeln!(ctx.io.output, "Repository {dir_name} already exists").ok();
    if confirm("Update existing repository?", args.yes, &mut ctx.io)? {
      writeln!(ctx.io.output, "Updating {dir_name}\n").ok();
      crate::time!(ctx.flags.timing, ctx.io.output, "Update", {
        pull_repository(&repo_path, ctx)?;
      });
    }
  } else {
    clone_repository(repo, base_dir, args.connection.ssh, ctx)?;
  }

  let build_path = repo_path.join(&args.build.build_dir);
  prepare_build_dir(&build_path, args.build.clean, ctx)?;

  let build_system = if let Some(bs) = args.build.build_system {
    Some(bs)
  } else if !ctx.flags.dry_run {
    Some(detect_build_system(&repo_path, ctx)?)
  } else {
    None
  };

  if let Some(build_system) = build_system {
    configure_and_build(args, &repo_path, &build_path, build_system, false, ctx)?;
  }

  writeln!(
    ctx.io.output,
    "Project finished in {dir_name}/{}",
    args.build.build_dir
  )
  .ok();

  if ctx.flags.timing {
    writeln!(ctx.io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
  Ok(())
}
