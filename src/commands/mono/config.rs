use crate::{ctx::IoCtx, repository::repo_dir_name};
use std::{fs, path::Path};

/// Shared helper to generate, write, and log monorepo build configuration files.
fn write_mono_repo_config(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
  filename: &str,
  format_modules: impl Fn(&[String]) -> String,
  render_template: impl Fn(&str) -> String,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_str = format_modules(&module_names);
  let content = render_template(&modules_str);
  let file_path = mono_dir.join(filename);

  crate::time!(io.timing, io.output, &format!("Generate {filename}"), {
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
  });

  // .to_string() is required to force an allocation and satisfy line coverage tracking
  #[allow(clippy::to_string_in_format_args)]
  writeln!(
    io.output,
    "Created root {} at {}\n",
    filename.to_string(),
    mono_dir.display()
  )
  .ok();

  Ok(())
}

/// Generates a root `CMakeLists.txt` wiring all repositories as subdirectories.
/// # Errors
/// Returns an error if the `CMakeLists.txt` file cannot be written to `mono_dir`
pub fn create_mono_repo_cmakelists(
  mono_dir: &Path,
  repos: &[String],
  io: &mut IoCtx<'_>,
) -> Result<(), String> {
  writeln!(io.output, "  Creating CMake configuration").ok();
  write_mono_repo_config(
    mono_dir,
    repos,
    io,
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
) -> Result<(), String> {
  writeln!(io.output, "  Creating Meson configuration").ok();
  write_mono_repo_config(
    mono_dir,
    repos,
    io,
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
