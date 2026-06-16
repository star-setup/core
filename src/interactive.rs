//! Interactive CLI mode for ecosystem-setup.

use std::io::{self, BufRead, Write};
use crate::cli::ResolvedArgs;

fn ask(prompt: &str) -> String {
  print!("{prompt}: ");
  io::stdout().flush().ok();
  let mut input = String::new();
  if io::stdin().lock().read_line(&mut input).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  input.trim().to_string()
}

fn ask_default(prompt: &str, default: &str) -> String {
  print!("{prompt} [{default}]: ");
  io::stdout().flush().ok();
  let mut input = String::new();
  if io::stdin().lock().read_line(&mut input).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  let val = input.trim().to_string();
  if val.is_empty() { default.to_string() } else { val }
}

fn ask_yesno(prompt: &str, default: bool) -> bool {
  let default_char = if default { "Y" } else { "N" };
  print!("{prompt} (y/n) [{default_char}]: ");
  io::stdout().flush().ok();
  let mut input = String::new();
  if io::stdin().lock().read_line(&mut input).unwrap_or(0) == 0 {
    eprintln!("\nError: unexpected end of input");
    std::process::exit(1);
  }
  let val = input.trim().to_lowercase();
  if val.is_empty() { default } else { val.starts_with('y') }
}

/// Interactive CLI mode — prompts for any unset arguments.
pub fn interactive_mode(args: &mut ResolvedArgs) {
  println!("Ecosystem Setup Interactive Mode");

  if args.repo.is_none() {
    loop {
      let repo = ask("Enter repository (user/repo or URL)");
      if !repo.is_empty() { args.repo = Some(repo); break; }
    }
  }

  if !args.connection.ssh {
    args.connection.ssh = ask_yesno("Use SSH?", false);
  }
  if !args.connection.verbose {
    args.connection.verbose = ask_yesno("Verbose?", false);
  }
  if !args.build.clean {
    args.build.clean = ask_yesno("Clean build directory if exists?", false);
  }

  if !args.mono.mono_repo {
    loop {
      let mode = ask("Select mode: (1) Single Repo (2) Mono-Repo");
      match mode.as_str() {
        "1" => { break; }
        "2" => { args.mono.mono_repo = true; break; }
        _ => {}
      }
    }
  }

  if args.mono.mono_repo && args.mono.profile.is_none() && args.mono.repos.is_none() {
    loop {
      let choice = ask("Mono-repo: (1) Use profile (2) Manual repo list");
      match choice.as_str() {
        "1" => {
          loop {
            let profile = ask("Profile name");
            if !profile.is_empty() { args.mono.profile = Some(profile); break; }
          }
          break;
        }
        "2" => {
          loop {
            let repo_list = ask(
              "Enter repos (space separated 'username/lib1 username/lib2')"
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

  args.build.build_type = ask_default("Build type", &args.build.build_type);
  args.build.build_dir = ask_default("Build directory", &args.build.build_dir);

  if args.cmake_flags.is_empty() {
    let cmake_extra = ask_default(
      "Additional CMake args (space separated)", ""
    );
    if !cmake_extra.is_empty() {
      args.cmake_flags = cmake_extra.split_whitespace().map(String::from).collect();
    }
  }

  if !args.build.no_build {
    args.build.no_build = ask_yesno("Configure only (skip build)?", false);
  }

  println!("\nInteractive mode complete");
}
