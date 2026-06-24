//! Interactive CLI mode.

use crate::cli::{build::BuildType, ResolvedArgs};
use crate::prompts::{ask, ask_default, ask_yesno};
use std::io::{BufRead, Write};

/// Interactive CLI mode — prompts for any unset arguments.
/// # Errors
/// Returns an error if stdin reaches EOF unexpectedly.
pub fn interactive_mode(
  args: &mut ResolvedArgs,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
  writeln!(output, "Star Setup Interactive Mode").ok();

  if args.repo.is_none() {
    loop {
      let repo = ask("Enter repository (user/repo or URL)", input, output)?;
      if !repo.is_empty() {
        args.repo = Some(repo);
        break;
      }
    }
  }

  if !args.connection.ssh {
    args.connection.ssh = ask_yesno("Use SSH?", false, input, output)?;
  }
  if !args.connection.verbose {
    args.connection.verbose = ask_yesno("Verbose?", false, input, output)?;
  }
  if !args.build.clean {
    args.build.clean = ask_yesno("Clean build directory if exists?", false, input, output)?;
  }

  if !args.mono.mono_repo {
    loop {
      let mode = ask("Select mode: (1) Single Repo (2) Mono-Repo", input, output)?;
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
      let choice = ask(
        "Mono-repo: (1) Use profile (2) Manual repo list",
        input,
        output,
      )?;
      match choice.as_str() {
        "1" => {
          loop {
            let profile = ask("Profile name", input, output)?;
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
              input,
              output,
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

  let build_type_str = ask_default(
    "Build type",
    args.build.build_type.to_cmake(),
    input,
    output,
  )?;
  args.build.build_type = build_type_str.parse::<BuildType>()?;
  args.build.build_dir = ask_default("Build directory", &args.build.build_dir, input, output)?;

  if args.build.cmake_flags.is_empty() {
    let cmake_extra = ask_default("Additional CMake args (space separated)", "", input, output)?;
    if !cmake_extra.is_empty() {
      args.build.cmake_flags = cmake_extra.split_whitespace().map(String::from).collect();
    }
  }

  if args.build.meson_flags.is_empty() {
    let meson_extra = ask_default("Additional Meson args (space separated)", "", input, output)?;
    if !meson_extra.is_empty() {
      args.build.meson_flags = meson_extra.split_whitespace().map(String::from).collect();
    }
  }

  if !args.build.no_build {
    args.build.no_build = ask_yesno("Configure only (skip build)?", false, input, output)?;
  }

  writeln!(output, "\nInteractive mode complete").ok();
  Ok(())
}
