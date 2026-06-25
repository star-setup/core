//! Interactive CLI mode.

use crate::{
  cli::{BuildType, ResolvedArgs},
  ctx::IoCtx,
  prompts::{ask, ask_default, ask_yesno},
};

/// Interactive CLI mode — prompts for any unset arguments.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn interactive_mode(args: &mut ResolvedArgs, io: &mut IoCtx<'_>) -> Result<(), String> {
  writeln!(io.output, "Star Setup Interactive Mode").ok();

  if args.repo.is_none() {
    loop {
      let repo = ask("Enter repository (user/repo or URL)", io)?;
      if !repo.is_empty() {
        args.repo = Some(repo);
        break;
      }
    }
  }

  if !args.connection.ssh {
    args.connection.ssh = ask_yesno("Use SSH?", false, io)?;
  }
  if !args.connection.verbose {
    args.connection.verbose = ask_yesno("Verbose?", false, io)?;
  }
  if !args.diagnostic.timing {
    args.diagnostic.timing = ask_yesno("Show timing?", false, io)?;
  }
  if !args.build.clean {
    args.build.clean = ask_yesno("Clean build directory if exists?", false, io)?;
  }

  if !args.mono.mono_repo {
    loop {
      let mode = ask("Select mode: (1) Single Repo (2) Mono-Repo", io)?;
      match mode.as_str() {
        "1" => break,
        "2" => {
          args.mono.mono_repo = true;
          break;
        }
        _ => {}
      }
    }
  }

  if args.mono.mono_repo && args.mono.profile.is_none() && args.mono.repos.is_none() {
    loop {
      let choice = ask("Mono-repo: (1) Use profile (2) Manual repo list", io)?;
      match choice.as_str() {
        "1" => {
          loop {
            let profile = ask("Profile name", io)?;
            if !profile.is_empty() {
              args.mono.profile = Some(profile);
              break;
            }
          }
          break;
        }
        "2" => {
          loop {
            let repo_list = ask(
              "Enter repos (space separated 'username/lib1 username/lib2')",
              io,
            )?;
            if !repo_list.is_empty() {
              args.mono.repos = Some(repo_list.split_whitespace().map(String::from).collect());
              break;
            }
          }
          break;
        }
        _ => {}
      }
    }
  }

  let build_type_str = ask_default("Build type", args.build.build_type.to_cmake(), io)?;
  args.build.build_type = build_type_str.parse::<BuildType>()?;
  args.build.build_dir = ask_default("Build directory", &args.build.build_dir, io)?;

  if !args.build.no_build {
    args.build.no_build = ask_yesno("Configure only (skip build)?", false, io)?;
  }

  writeln!(io.output, "\nInteractive mode complete").ok();
  Ok(())
}
