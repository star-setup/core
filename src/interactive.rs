//! Interactive CLI mode.

use crate::cli::ResolvedArgs;
use std::io::{BufRead, Write};

/// Prompts the user for a required string value.
fn ask(prompt: &str, input: &mut impl BufRead, output: &mut impl Write) -> String {
  write!(output, "{prompt}: ").ok();
  output.flush().ok();
  let mut line = String::new();
  if input.read_line(&mut line).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  line.trim().to_string()
}

/// Prompts the user for a string value, returning `default` if the input is empty.
fn ask_default(
  prompt: &str,
  default: &str,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> String {
  write!(output, "{prompt} [{default}]: ").ok();
  output.flush().ok();
  let mut line = String::new();
  if input.read_line(&mut line).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  let val = line.trim().to_string();
  if val.is_empty() {
    default.to_string()
  } else {
    val
  }
}

/// Prompts the user for a yes/no answer, returning `default` if the input is empty.
fn ask_yesno(
  prompt: &str,
  default: bool,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> bool {
  let default_char = if default { "Y" } else { "N" };
  write!(output, "{prompt} (y/n) [{default_char}]: ").ok();
  output.flush().ok();
  let mut line = String::new();
  if input.read_line(&mut line).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  let val = line.trim().to_lowercase();
  if val.is_empty() {
    default
  } else {
    val.starts_with('y')
  }
}

/// Interactive CLI mode — prompts for any unset arguments.
pub fn interactive_mode(
  args: &mut ResolvedArgs,
  input: &mut impl BufRead,
  output: &mut impl Write,
) {
  writeln!(output, "Star Setup Interactive Mode").ok();

  if args.repo.is_none() {
    loop {
      let repo = ask("Enter repository (user/repo or URL)", input, output);
      if !repo.is_empty() {
        args.repo = Some(repo);
        break;
      }
    }
  }

  if !args.connection.ssh {
    args.connection.ssh = ask_yesno("Use SSH?", false, input, output);
  }
  if !args.connection.verbose {
    args.connection.verbose = ask_yesno("Verbose?", false, input, output);
  }
  if !args.build.clean {
    args.build.clean = ask_yesno("Clean build directory if exists?", false, input, output);
  }

  if !args.mono.mono_repo {
    loop {
      let mode = ask("Select mode: (1) Single Repo (2) Mono-Repo", input, output);
      match mode.as_str() {
        "1" => {
          break;
        }
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
      );
      match choice.as_str() {
        "1" => {
          loop {
            let profile = ask("Profile name", input, output);
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
            );
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

  args.build.build_type = ask_default("Build type", &args.build.build_type, input, output);
  args.build.build_dir = ask_default("Build directory", &args.build.build_dir, input, output);

  if args.cmake_flags.is_empty() {
    let cmake_extra = ask_default("Additional CMake args (space separated)", "", input, output);
    if !cmake_extra.is_empty() {
      args.cmake_flags = cmake_extra.split_whitespace().map(String::from).collect();
    }
  }

  if !args.build.no_build {
    args.build.no_build = ask_yesno("Configure only (skip build)?", false, input, output);
  }

  writeln!(output, "\nInteractive mode complete").ok();
}
