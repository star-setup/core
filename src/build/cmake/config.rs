use crate::{
  build::common::write_mono_repo_config,
  ctx::{IoCtx, RunFlags},
};
use std::path::Path;

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
