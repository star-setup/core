use crate::repository::repo_dir_name;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Generates a root `CMakeLists.txt` wiring all repositories as subdirectories.
/// # Errors
/// Returns an error if the `CMakeLists.txt` file cannot be written to `mono_dir`
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
