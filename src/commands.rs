//! Command handlers for single and mono-repo modes.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::cli::ResolvedArgs;
use crate::config::EcosystemConfig;
use crate::profiles::list_profiles;
use crate::repository::{resolve_repo_url, clone_repository, repo_name};
use crate::utils::run_command;

fn print_mode_header(
  mode: &str,
  test_repo: Option<&str>,
  repo_name: Option<&str>,
  use_ssh: bool,
  mono_dir: Option<&str>,
  profile: Option<&str>,
  lib_count: Option<usize>,
) {
  println!("Ecosystem Setup: {mode}");
  if      let Some(p) = profile    { println!("  Profile: {p}");         }
  if      let Some(r) = test_repo  { println!("  Test Repository: {r}"); }
  else if let Some(r) = repo_name  { println!("  Repository: {r}");      }
  println!("  Clone Method: {}", if use_ssh { "SSH" } else { "HTTPS" });
  if      let Some(d) = mono_dir   { println!("  Directory: {d}");       }
  if      let Some(c) = lib_count { println!("  Libraries: {c}");       }
  println!();
}

pub fn single_repo_mode(args: &ResolvedArgs) -> Result<(), String> {
  let repo = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_url = resolve_repo_url(repo, args.connection.ssh);
  let repo_name = repo_name(&repo_url);

  print_mode_header(
    "Single Repository Mode",
    None,
    Some(&repo_name),
    args.connection.ssh,
    None,
    None,
    None
  );

  let repo_path = Path::new(&repo_name);
  if repo_path.exists() {
    println!("Repository {repo_name} already exists");
    print!("Update existing repository? (y/n): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    if input.trim().eq_ignore_ascii_case("y") {
      println!("Updating {repo_name}\n");
      run_command(&["git", "pull"], Some(Path::new(&repo_name)), args.connection.verbose)?;
    }
  } else {
    println!("Cloning {repo_name}\n");
    run_command(&["git", "clone", &repo_url], None, args.connection.verbose)?;
  }

  let build_path = PathBuf::from(&repo_name).join(&args.build.build_dir);
  if args.build.clean && build_path.exists() {
    println!("Cleaning build directory\n");
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  println!("Creating build directory: {}\n", args.build.build_dir);
  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  println!("Configuring with CMake\n");
  let build_type = format!("-DCMAKE_BUILD_TYPE={}", args.build.build_type);
  let mut cmake_cmd = vec!["cmake", "..", &build_type];
  cmake_cmd.extend(args.cmake_flags.iter().map(String::as_str));
  run_command(&cmake_cmd, Some(build_path.as_path()), args.connection.verbose)?;

  if !args.build.no_build {
    println!("Building project\n");
    run_command(
      &["cmake", "--build", ".", "--config", &args.build.build_type],
      Some(build_path.as_path()),
      args.connection.verbose
    )?;
  }

  println!("Project finished in {repo_name}/{}", args.build.build_dir);
  Ok(())
}

fn create_mono_repo_cmakelists(
  mono_dir: &Path,
  test_repo: &str,
  repos: &[String]
) -> Result<(), String> {
  let module_names: Vec<&str> = repos
    .iter()
    .map(|r| r
    .split('/')
    .next_back()
    .unwrap_or(r.as_str())
    .trim_end_matches(".git"))
    .collect();
  let modules_cmake = module_names.join("\n  ");

  let cmake_content = format!("
cmake_minimum_required(VERSION 3.23)

project(ecosystem_dev LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)

if(NOT EXISTS \"${{CMAKE_CURRENT_SOURCE_DIR}}/{test_repo}/CMakeLists.txt\")
  message(FATAL_ERROR \"Test repository '{test_repo}' not found\")
endif()

set(MONO_REPO_MODULES
  {modules_cmake}
)

foreach(module IN LISTS MONO_REPO_MODULES)
  if(EXISTS \"${{CMAKE_CURRENT_SOURCE_DIR}}/${{module}}/CMakeLists.txt\")
    add_subdirectory(${{module}})
  else()
    message(WARNING \"Module ${{module}} not found or missing CMakeLists.txt\")
  endif()
endforeach()

set_property(GLOBAL PROPERTY USE_FOLDERS ON)
set_property(GLOBAL PROPERTY PREDEFINED_TARGETS_FOLDER \"External\")

string(REPLACE \"-\" \"_\" target \"{test_repo}\")
set_property(DIRECTORY ${{CMAKE_CURRENT_SOURCE_DIR}} PROPERTY VS_STARTUP_PROJECT ${{target}})
");

  let cmake_file = mono_dir.join("CMakeLists.txt");
  fs::write(&cmake_file, cmake_content).map_err(|e| e.to_string())?;
  println!("Created root CMakeLists.txt at {}\n", mono_dir.display());
  Ok(())
}

pub fn mono_repo_mode(args: &ResolvedArgs, config: &EcosystemConfig) -> Result<(), String> {
  let repo_input = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_input = repo_input.trim_end_matches('/');

  let test_repo = if repo_input.starts_with("http") || repo_input.starts_with("git@") {
    if repo_input.contains("github.com/") || repo_input.contains("github.com:") {
      let parts: Vec<&str> = repo_input.split('/').collect();
      let user = parts[parts.len()-2].split(':').next_back().unwrap_or("");
      let repo = parts[parts.len()-1].trim_end_matches(".git");
      format!("{user}/{repo}")
    } else {
      return Err("Could not parse repository URL".to_string());
    }
  } else if repo_input.contains('/') {
    repo_input.to_string()
  } else {
    return Err("Repository must be in format 'username/repo' for mono-repo mode".to_string());
  };

  let test_repo_name = repo_name(&test_repo);

  let mut repos: Vec<String> = if let Some(profile_name) = &args.mono.profile {
    let profile_repos = config.profiles.get(profile_name)
      .ok_or_else(|| { list_profiles(config); format!("Profile '{profile_name}' not found") })?;
    if profile_repos.is_empty() {
      return Err(format!("Profile '{profile_name}' has no repositories"));
    }
    print_mode_header(
      "Profile",
      Some(&test_repo),
      None,
      args.connection.ssh,
      Some(&args.mono.mono_dir),
      Some(profile_name),
      Some(profile_repos.len())
    );
    profile_repos.clone()
  } else if let Some(r) = &args.mono.repos {
    print_mode_header(
      "Mono-repository",
      Some(&test_repo),
      None,
      args.connection.ssh,
      Some(&args.mono.mono_dir),
      None,
      Some(r.len())
    );
    r.clone()
  } else {
    return Err("No repos or profile specified for mono-repo mode".to_string());
  };

  if !repos.contains(&test_repo) { repos.push(test_repo.clone()); }

  println!("Total repositories: {}\n", repos.len());

  let mono_repo_path = PathBuf::from(&args.mono.mono_dir);
  println!("Creating directory: {}\n", mono_repo_path.display());
  fs::create_dir_all(&mono_repo_path).map_err(|e| e.to_string())?;

  println!("Cloning repositories");
  for repo in &repos {
    clone_repository(
      repo,
      &mono_repo_path,
      args.connection.ssh,
      args.connection.verbose
    )?;
  }
  println!("\n  Finished cloning ({} repositories)\n", repos.len());

  println!("Creating mono-repo configuration");
  create_mono_repo_cmakelists(&mono_repo_path, &test_repo_name, &repos)?;

  println!("Creating build directory\n");
  let build_path = mono_repo_path.join(&args.build.build_dir);
  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  println!("Configuring with CMake in {}\n", build_path.display());
  let build_type_flag = format!("-DCMAKE_BUILD_TYPE={}", args.build.build_type);
  let mut cmake_cmd = vec!["cmake", "-DBUILD_LOCAL=ON", &build_type_flag, ".."];
  cmake_cmd.extend(args.cmake_flags.iter().map(String::as_str));
  run_command(&cmake_cmd, Some(build_path.as_path()), args.connection.verbose)?;

  if !args.build.no_build {
    println!("Building project\n");
    run_command(
      &["cmake", "--build", ".", "--config", &args.build.build_type],
      Some(build_path.as_path()),
      args.connection.verbose
    )?;
  }

  println!("Setup complete");
  println!("Repositories in: {}", mono_repo_path.canonicalize().unwrap_or(mono_repo_path).display());
  println!("Build output in: {}", build_path.canonicalize().unwrap_or(build_path).display());
  Ok(())
}
