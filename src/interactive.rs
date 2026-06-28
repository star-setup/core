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
    args.repo = Some(ask_required("Enter repository (user/repo or URL)", io)?);
  }

  args.connection.ssh = ask_bool_if("Use SSH?", args.connection.ssh, io)?;

  args.connection.verbose = ask_bool_if("Verbose?", args.connection.verbose, io)?;
  io.verbose = args.connection.verbose;

  args.diagnostic.timing = ask_bool_if("Show timing?", args.diagnostic.timing, io)?;
  io.timing = args.diagnostic.timing;

  args.build.clean = ask_bool_if("Clean build directory if exists?", args.build.clean, io)?;

  if !args.mono.mono_repo {
    loop {
      match ask("Select mode: (1) Single Repo (2) Mono-Repo", io)?.as_str() {
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
      match ask("Mono-repo: (1) Use profile (2) Manual repo list", io)?.as_str() {
        "1" => {
          args.mono.profile = Some(ask_required("Profile name", io)?);
          break;
        }
        "2" => {
          let repo_list = ask_required(
            "Enter repos (space separated 'username/lib1 username/lib2')",
            io,
          )?;
          args.mono.repos = Some(repo_list.split_whitespace().map(String::from).collect());
          break;
        }
        _ => {}
      }
    }
  }

  let build_type_str = ask_default("Build type", args.build.build_type.to_cmake(), io)?;
  args.build.build_type = build_type_str.parse::<BuildType>()?;
  args.build.build_dir = ask_default("Build directory", &args.build.build_dir, io)?;

  args.build.no_build = ask_bool_if("Configure only (skip build)?", args.build.no_build, io)?;

  writeln!(io.output, "\nInteractive mode complete").ok();
  Ok(())
}

/// Helper to ask a boolean question only if the condition isn't already met.
fn ask_bool_if(prompt: &str, current_val: bool, io: &mut IoCtx<'_>) -> Result<bool, String> {
  if !current_val {
    ask_yesno(prompt, false, io)
  } else {
    Ok(current_val)
  }
}

/// Helper to repeatedly prompt until a non-empty string is provided.
fn ask_required(prompt: &str, io: &mut IoCtx<'_>) -> Result<String, String> {
  loop {
    let response = ask(prompt, io)?;
    if !response.is_empty() {
      return Ok(response);
    }
  }
}
