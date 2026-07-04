use crate::{
  ctx::{IoCtx, RunCtx},
  prompts::{
    confirm, confirm_batch,
    BatchConfirm::{No, NoAll, Yes, YesAll},
  },
  repository::{pull_repo, repo_dir_name, resolve_repo_url},
};
use std::{
  fs::{read_dir, remove_dir_all},
  path::Path,
};

fn is_dir_empty(dir: &Path) -> bool {
  read_dir(dir).is_ok_and(|mut entries| entries.next().is_none())
}

/// Returns `true` if the directory contains a `.git` entry.
fn is_git_repo(dir: &Path) -> bool {
  dir.join(".git").exists()
}

pub enum ExistsAction {
  Skip,
  Update,
}

/// Clones a single repository into the target directory.
/// Skips if the repository already exists.
/// # Errors
/// Returns an error if the git clone command fails, an existing invalid directory
/// cannot be removed, or the `on_exists` callback returns an error.
pub fn clone_repo(
  repo_path: &str,
  target_dir: &Path,
  use_ssh: bool,
  on_exists: impl FnOnce(&mut IoCtx<'_>) -> Result<ExistsAction, String>,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let repo_name = repo_dir_name(repo_path);
  let repo_dir = target_dir.join(&repo_name);

  if repo_dir.exists() && is_git_repo(&repo_dir) {
    writeln!(ctx.io.output, "  Repository already exists").ok();
    if matches!(on_exists(&mut ctx.io)?, ExistsAction::Update) {
      crate::time!(ctx.flags.timing, ctx.io.output, "Update", {
        pull_repo(&repo_dir, ctx)?;
      });
    }
    return Ok(());
  }

  if repo_dir.exists() && !is_dir_empty(&repo_dir) {
    writeln!(
      ctx.io.output,
      "  Directory exists but is not a git repository"
    )
    .ok();
    if !confirm(
      &format!(
        "  Remove it and re-clone? WARNING: This will delete all files in {}",
        repo_dir.display()
      ),
      false,
      &mut ctx.io,
    )? {
      writeln!(ctx.io.output, "  Skipping {repo_name}").ok();
      return Ok(());
    }

    if ctx.flags.dry_run {
      writeln!(
        ctx.io.output,
        "  Would remove directory: {}",
        repo_dir.display()
      )
      .ok();
    } else {
      remove_dir_all(&repo_dir)
        .map_err(|e| format!("Failed to remove {}: {e}", repo_dir.display()))?;
    }
  }

  let repo_url = resolve_repo_url(repo_path, use_ssh);
  crate::time!(ctx.flags.timing, ctx.io.output, "Clone", {
    ctx.runner.run(
      &["git", "clone", &repo_url, &repo_name],
      Some(target_dir),
      ctx.flags,
      ctx.io.output,
    )?;
    if ctx.flags.verbose {
      writeln!(ctx.io.output, "  Finished cloning {repo_name}").ok();
    }
    Ok::<(), String>(())
  })
  .map_err(|e| format!("Failed to clone {repo_path}: {e}"))?;

  Ok(())
}

/// Clones all repositories into the given directory, prompting before updating
/// any that already exist (with all/skip-all shortcuts for 5 or more repos).
/// # Errors
/// Returns an error if any repository fails to clone, an existing invalid
/// directory cannot be removed, or a confirmation prompt reaches end of input.
pub fn clone_repos(
  repos: &[String],
  target_dir: &Path,
  ssh: bool,
  yes: bool,
  ctx: &mut RunCtx<'_, '_>,
) -> Result<(), String> {
  let allow_all = repos.len() >= 5;
  let mut remembered: Option<bool> = None;
  writeln!(ctx.io.output, "Cloning repositories").ok();
  crate::time!(ctx.flags.timing, ctx.io.output, "Clone", {
    for repo in repos {
      writeln!(ctx.io.output, "  Cloning {}", repo_dir_name(repo)).ok();
      clone_repo(
        repo,
        target_dir,
        ssh,
        |io| {
          if yes {
            return Ok(ExistsAction::Update);
          }
          if let Some(update) = remembered {
            return Ok(if update {
              ExistsAction::Update
            } else {
              ExistsAction::Skip
            });
          }
          Ok(
            match confirm_batch("  Update existing repository?", allow_all, io)? {
              Yes => ExistsAction::Update,
              No => ExistsAction::Skip,
              YesAll => {
                remembered = Some(true);
                ExistsAction::Update
              }
              NoAll => {
                remembered = Some(false);
                ExistsAction::Skip
              }
            },
          )
        },
        ctx,
      )?;
    }
    if ctx.flags.verbose {
      writeln!(
        ctx.io.output,
        "  Finished cloning ({} repositories)",
        repos.len()
      )
      .ok();
    }
    Ok::<(), String>(())
  })?;
  writeln!(ctx.io.output).ok();
  Ok(())
}
