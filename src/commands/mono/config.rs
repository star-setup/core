use crate::{
  ctx::{IoCtx, RunFlags},
  repository::repo_dir_name,
  utils::dry_run_or_do,
};
use serde_json::{from_str, Value};
use std::{
  fs::{self, read_to_string},
  path::Path,
};

/// Shared helper to generate, write, and log monorepo build configuration files.
fn write_mono_repo_config(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
  filename: &str,
  format_modules: impl Fn(&[String]) -> String,
  render_template: impl Fn(&str) -> String,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_str = format_modules(&module_names);
  let content = render_template(&modules_str);
  let file_path = mono_dir.join(filename);

  dry_run_or_do(
    "create file",
    "Creating",
    &file_path,
    io,
    flags,
    &format!("Generate {filename}"),
    || fs::write(&file_path, content).map_err(|e| e.to_string()),
  )?;

  // .to_string() is required to force an allocation and satisfy line coverage tracking
  if flags.dry_run {
    if flags.verbose {
      #[allow(clippy::to_string_in_format_args)]
      writeln!(
        io.output,
        "  Would create root {} at {}\n",
        filename.to_string(),
        mono_dir.display()
      )
      .ok();
    }
  } else {
    #[allow(clippy::to_string_in_format_args)]
    writeln!(
      io.output,
      "  Created root {} at {}\n",
      filename.to_string(),
      mono_dir.display()
    )
    .ok();
  }

  Ok(())
}

/// Generates a root `CMakeLists.txt` wiring all repositories as subdirectories.
/// # Errors
/// Returns an error if the `CMakeLists.txt` file cannot be written to `mono_dir`
pub fn create_mono_repo_cmakelists(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  writeln!(io.output, "  Creating CMake configuration").ok();
  write_mono_repo_config(
    mono_dir,
    repos,
    io,
    flags,
    "CMakeLists.txt",
    |modules| modules.join("\n  "),
    |modules_cmake| {
      format!(
        "cmake_minimum_required(VERSION 3.23)

project(star_setup LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)

set(MONO_REPO_MODULES
  {modules_cmake}
)

foreach(module IN LISTS MONO_REPO_MODULES)
  if(EXISTS \"${{CMAKE_CURRENT_SOURCE_DIR}}/repos/${{module}}/CMakeLists.txt\")
    add_subdirectory(repos/${{module}})
  else()
    message(WARNING \"Module ${{module}} not found or missing CMakeLists.txt\")
  endif()
endforeach()

set_property(GLOBAL PROPERTY USE_FOLDERS ON)
set_property(GLOBAL PROPERTY PREDEFINED_TARGETS_FOLDER \"External\")
"
      )
    },
  )
}

/// Generates a root `meson.build` wiring all repositories as subprojects.
/// # Errors
/// Returns an error if the `meson.build` file cannot be written to `mono_dir`.
pub fn create_mono_repo_mesonbuild(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  writeln!(io.output, "  Creating Meson configuration").ok();
  write_mono_repo_config(
    mono_dir,
    repos,
    io,
    flags,
    "meson.build",
    |modules| {
      modules
        .iter()
        .map(|m| format!("  '{m}'"))
        .collect::<Vec<_>>()
        .join(",\n")
    },
    |modules_meson| {
      format!(
        "project('star_setup', 'cpp',
  default_options: ['cpp_std=c++20'],
  subproject_dir: 'repos'
)

modules = [
{modules_meson},
]

foreach module : modules
  subproject(module)
endforeach
"
      )
    },
  )
}

/// Generates a root `package.json` wiring all repositories as npm workspaces.
/// # Errors
/// Returns an error if the `package.json` file cannot be written to `mono_dir`.
pub fn create_mono_repo_package_json(
  mono_dir: &Path,
  repos_path: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  flags: RunFlags,
) -> Result<(), String> {
  writeln!(io.output, "  Creating npm workspace configuration").ok();

  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();

  let workspaces = module_names
    .iter()
    .map(|m| format!("    \"repos/{m}\""))
    .collect::<Vec<_>>()
    .join(",\n");

  // Read package name from each lib repo (skip first — test repo)
  let mut overrides = Vec::new();
  for (i, dir) in module_names.iter().enumerate() {
    if i == 0 {
      continue;
    }
    let pkg_path = repos_path.join(dir).join("package.json");
    if let Ok(content) = read_to_string(&pkg_path) {
      match from_str::<Value>(&content) {
        Ok(json) => {
          if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
            overrides.push(format!("    \"{name}\": \"*\""));
          }
        }
        Err(_) => {
          if flags.verbose {
            writeln!(
              io.output,
              "  Warning: malformed {dir}/package.json, skipping override"
            )
            .ok();
          }
        }
      }
    } else if flags.verbose {
      writeln!(
        io.output,
        "  Warning: could not read {dir}/package.json, skipping override"
      )
      .ok();
    }
  }

  let overrides_block = if overrides.is_empty() {
    String::new()
  } else {
    format!(",\n  \"overrides\": {{\n{}\n  }}", overrides.join(",\n"))
  };

  let content = format!(
    "{{\n  \"name\": \"star-setup-workspace\",\n  \"private\": true,\n  \"workspaces\": [\n{workspaces}\n  ]{overrides_block}\n}}\n"
  );

  let file_path = mono_dir.join("package.json");
  dry_run_or_do(
    "create file",
    "Creating",
    &file_path,
    io,
    flags,
    "Generate package.json",
    || fs::write(&file_path, content).map_err(|e| e.to_string()),
  )?;

  if flags.dry_run {
    if flags.verbose {
      writeln!(
        io.output,
        "  Would create root package.json at {}\n",
        mono_dir.display()
      )
      .ok();
    }
  } else {
    writeln!(
      io.output,
      "  Created root package.json at {}\n",
      mono_dir.display()
    )
    .ok();
  }

  Ok(())
}
