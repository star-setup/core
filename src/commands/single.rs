use crate::{
  cli::ResolvedArgs,
  commands::{
    build::build_project,
    header::{print_mode_header, ModeHeader},
  },
  ctx::RunCtx,
  prompts::confirm,
  repository::{repo_dir_name, resolve_repo_url},
};
use std::{
  fs,
  path::{Path, PathBuf},
};

/// Clones and configures a single repository.
/// # Errors
/// Returns an error if no repository is specified, or if any git or build system fails.
pub fn single_repo_mode(args: &ResolvedArgs, ctx: &mut RunCtx<'_>) -> Result<(), String> {
  let total = std::time::Instant::now();
  let repo = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_url = resolve_repo_url(repo, args.connection.ssh);
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
    ctx.io.output,
  );

  let repo_path = Path::new(&dir_name);
  if repo_path.exists() {
    writeln!(ctx.io.output, "Repository {dir_name} already exists").ok();
    if confirm(
      "Update existing repository?",
      args.yes,
      ctx.io.input,
      ctx.io.output,
    )? {
      writeln!(ctx.io.output, "Updating {dir_name}\n").ok();
      crate::time!(args.diagnostic.timing, ctx.io.output, "Update", {
        ctx
          .runner
          .run(&["git", "pull"], Some(Path::new(&dir_name)), ctx.io.output)?;
      });
    }
  } else {
    writeln!(ctx.io.output, "Cloning {dir_name}\n").ok();
    crate::time!(args.diagnostic.timing, ctx.io.output, "Clone", {
      ctx
        .runner
        .run(&["git", "clone", &repo_url, &dir_name], None, ctx.io.output)?;
    });
  }

  let build_path = PathBuf::from(&dir_name).join(&args.build.build_dir);
  if args.build.clean && build_path.exists() {
    writeln!(ctx.io.output, "Cleaning build directory\n").ok();
    crate::time!(ctx.io.timing, ctx.io.output, "Clean", {
      fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
    });
  }

  writeln!(
    ctx.io.output,
    "Creating build directory: {}\n",
    args.build.build_dir
  )
  .ok();
  crate::time!(
    args.diagnostic.timing,
    ctx.io.output,
    "Create build directory",
    {
      fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;
    }
  );

  writeln!(ctx.io.output, "Configuring project\n").ok();
  build_project(args, build_path.as_path(), Path::new(&dir_name), false, ctx)?;

  writeln!(
    ctx.io.output,
    "Project finished in {dir_name}/{}",
    args.build.build_dir
  )
  .ok();

  if ctx.io.timing {
    writeln!(ctx.io.output, "[timing] Total: {:.2?}", total.elapsed()).ok();
  }
  Ok(())
}
