//! Command handlers for single and mono-repo modes.

use crate::cli::ResolvedArgs;
use crate::config::SetupConfig;
use crate::profiles::list_profiles;
use crate::repository::{clone_repository, repo_dir_name, resolve_repo_url};
use crate::utils::{confirm, run_command};
use std::fs;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

struct ModeHeader<'a> {
  mode: &'a str,
  test_repo: Option<&'a str>,
  repo_name: Option<&'a str>,
  use_ssh: bool,
  mono_dir: Option<&'a str>,
  profile: Option<&'a str>,
  lib_count: Option<usize>,
}

fn print_mode_header(header: &ModeHeader<'_>, output: &mut impl Write) {
  writeln!(output, "Star Setup: {}", header.mode).ok();
  if let Some(p) = header.profile {
    writeln!(output, "  Profile: {p}").ok();
  }
  if let Some(r) = header.test_repo {
    writeln!(output, "  Test Repository: {r}").ok();
  } else if let Some(r) = header.repo_name {
    writeln!(output, "  Repository: {r}").ok();
  }
  writeln!(
    output,
    "  Clone Method: {}",
    if header.use_ssh { "SSH" } else { "HTTPS" }
  )
  .ok();
  if let Some(d) = header.mono_dir {
    writeln!(output, "  Directory: {d}").ok();
  }
  if let Some(c) = header.lib_count {
    writeln!(output, "  Libraries: {c}").ok();
  }
  writeln!(output).ok();
}

fn cmake_build(
  args: &ResolvedArgs,
  build_path: &Path,
  mono: bool,
  output: &mut impl Write,
) -> Result<(), String> {
  let build_type_flag = format!("-DCMAKE_BUILD_TYPE={}", args.build.build_type);
  let mut cmake_cmd = if mono {
    vec!["cmake", "-DBUILD_LOCAL=ON", &build_type_flag, ".."]
  } else {
    vec!["cmake", "..", &build_type_flag]
  };
  cmake_cmd.extend(args.cmake_flags.iter().map(String::as_str));
  run_command(
    &cmake_cmd,
    Some(build_path),
    args.connection.verbose,
    output,
  )?;
  if !args.build.no_build {
    println!("Building project\n");
    run_command(
      &["cmake", "--build", ".", "--config", &args.build.build_type],
      Some(build_path),
      args.connection.verbose,
      output,
    )?;
  }
  Ok(())
}

pub fn single_repo_mode(
  args: &ResolvedArgs,
  input: &mut impl BufRead,
  output: &mut impl Write,
) -> Result<(), String> {
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
    output,
  );

  let repo_path = Path::new(&dir_name);
  if repo_path.exists() {
    writeln!(output, "Repository {dir_name} already exists").ok();
    if confirm("Update existing repository?", args.yes, input, output) {
      writeln!(output, "Updating {dir_name}\n").ok();
      run_command(
        &["git", "pull"],
        Some(Path::new(&dir_name)),
        args.connection.verbose,
        output,
      )?;
    }
  } else {
    writeln!(output, "Cloning {dir_name}\n").ok();
    run_command(
      &["git", "clone", &repo_url, &dir_name],
      None,
      args.connection.verbose,
      output,
    )?;
  }

  let build_path = PathBuf::from(&dir_name).join(&args.build.build_dir);
  if args.build.clean && build_path.exists() {
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  writeln!(
    output,
    "Creating build directory: {}\n",
    args.build.build_dir
  )
  .ok();
  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  writeln!(output, "Configuring with CMake\n").ok();
  cmake_build(args, build_path.as_path(), false, output)?;

  writeln!(
    output,
    "Project finished in {dir_name}/{}",
    args.build.build_dir
  )
  .ok();
  Ok(())
}

pub fn resolve_test_repo(repo_input: &str) -> Result<String, String> {
  let repo_input = repo_input.trim_end_matches('/');
  if repo_input.starts_with("http") || repo_input.starts_with("git@") {
    if repo_input.contains("github.com/") || repo_input.contains("github.com:") {
      let parts: Vec<&str> = repo_input.split('/').collect();
      let user = parts[parts.len() - 2].split(':').next_back().unwrap_or("");
      let repo = parts[parts.len() - 1].trim_end_matches(".git");
      Ok(format!("{user}/{repo}"))
    } else {
      Err("Could not parse repository URL".to_string())
    }
  } else if repo_input.contains('/') {
    Ok(repo_input.to_string())
  } else {
    Err("Repository must be in format 'username/repo' for mono-repo mode".to_string())
  }
}

pub fn create_mono_repo_cmakelists(
  mono_dir: &Path,
  test_repo: &str,
  repos: &[String],
  output: &mut impl Write,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_cmake = module_names.join("\n  ");

  let cmake_content = format!(
    "
cmake_minimum_required(VERSION 3.23)

project(star_setup LANGUAGES CXX)
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
"
  );

  let cmake_file = mono_dir.join("CMakeLists.txt");
  fs::write(&cmake_file, cmake_content).map_err(|e| e.to_string())?;
  writeln!(
    output,
    "Created root CMakeLists.txt at {}\n",
    mono_dir.display()
  )
  .ok();
  Ok(())
}

fn resolve_repos_for_mono(
  args: &ResolvedArgs,
  config: &SetupConfig,
  test_repo: &str,
  output: &mut impl Write,
) -> Result<Vec<String>, String> {
  if let Some(profile_name) = &args.mono.profile {
    let profile_repos = config.profiles.get(profile_name).ok_or_else(|| {
      list_profiles(config, output);
      format!("Profile '{profile_name}' not found")
    })?;
    if profile_repos.is_empty() {
      return Err(format!("Profile '{profile_name}' has no repositories"));
    }
    print_mode_header(
      &ModeHeader {
        mode: "Profile",
        test_repo: Some(test_repo),
        repo_name: None,
        use_ssh: args.connection.ssh,
        mono_dir: Some(&args.mono.mono_dir),
        profile: Some(profile_name),
        lib_count: Some(profile_repos.len()),
      },
      output,
    );
    Ok(profile_repos.clone())
  } else if let Some(r) = &args.mono.repos {
    print_mode_header(
      &ModeHeader {
        mode: "Mono-repository",
        test_repo: Some(test_repo),
        repo_name: None,
        use_ssh: args.connection.ssh,
        mono_dir: Some(&args.mono.mono_dir),
        profile: None,
        lib_count: Some(r.len()),
      },
      output,
    );
    Ok(r.clone())
  } else {
    Err("No repos or profile specified for mono-repo mode".to_string())
  }
}

pub fn mono_repo_mode(
  args: &ResolvedArgs,
  config: &SetupConfig,
  output: &mut impl Write,
) -> Result<(), String> {
  let repo_input = args.repo.as_deref().ok_or("No repository specified")?;
  let repo_input = repo_input.trim_end_matches('/');

  let test_repo = resolve_test_repo(repo_input)?;
  let test_repo_name = repo_dir_name(&test_repo);

  let mut repos: Vec<String> = resolve_repos_for_mono(args, config, &test_repo, output)?;

  if !repos
    .iter()
    .any(|r| repo_dir_name(r) == repo_dir_name(&test_repo))
  {
    repos.push(test_repo.clone());
  }

  writeln!(output, "Total repositories: {}\n", repos.len()).ok();

  let mono_repo_path = PathBuf::from(&args.mono.mono_dir);
  writeln!(output, "Creating directory: {}\n", mono_repo_path.display()).ok();
  fs::create_dir_all(&mono_repo_path).map_err(|e| e.to_string())?;

  writeln!(output, "Cloning repositories").ok();
  for repo in &repos {
    clone_repository(
      repo,
      &mono_repo_path,
      args.connection.ssh,
      args.connection.verbose,
      output,
    )?;
  }
  writeln!(
    output,
    "\n  Finished cloning ({} repositories)\n",
    repos.len()
  )
  .ok();

  writeln!(output, "Creating mono-repo configuration").ok();
  create_mono_repo_cmakelists(&mono_repo_path, &test_repo_name, &repos, output)?;

  writeln!(output, "Creating build directory\n").ok();
  let build_path = mono_repo_path.join(&args.build.build_dir);

  if args.build.clean && build_path.exists() {
    writeln!(output, "Cleaning build directory\n").ok();
    fs::remove_dir_all(&build_path).map_err(|e| e.to_string())?;
  }

  fs::create_dir_all(&build_path).map_err(|e| e.to_string())?;

  writeln!(
    output,
    "Configuring with CMake in {}\n",
    build_path.display()
  )
  .ok();
  cmake_build(args, build_path.as_path(), true, output)?;

  writeln!(output, "Setup complete").ok();
  writeln!(
    output,
    "Repositories in: {}",
    dunce::canonicalize(&mono_repo_path)
      .unwrap_or(mono_repo_path)
      .display()
  )
  .ok();
  writeln!(
    output,
    "Build output in: {}",
    dunce::canonicalize(&build_path)
      .unwrap_or(build_path)
      .display()
  )
  .ok();
  Ok(())
}
