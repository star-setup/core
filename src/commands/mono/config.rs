use crate::repository::repo_dir_name;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Generates a root `CMakeLists.txt` wiring all repositories as subdirectories.
/// # Errors
/// Returns an error if the `CMakeLists.txt` file cannot be written to `mono_dir`
pub fn create_mono_repo_cmakelists(
  mono_dir: &Path,
  repos: &[String],
  output: &mut impl Write,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_cmake = module_names.join("\n  ");
  let cmake_content = format!(
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

/// Generates a root `meson.build` wiring all repositories as subprojects.
/// # Errors
/// Returns an error if the `meson.build` file cannot be written to `mono_dir`.
pub fn create_mono_repo_mesonbuild(
  mono_dir: &Path,
  repos: &[String],
  output: &mut impl Write,
) -> Result<(), String> {
  let module_names: Vec<String> = repos.iter().map(|r| repo_dir_name(r)).collect();
  let modules_meson = module_names
    .iter()
    .map(|m| format!("  '{m}'"))
    .collect::<Vec<_>>()
    .join(",\n");
  let meson_content = format!(
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
  );
  let meson_file = mono_dir.join("meson.build");
  fs::write(&meson_file, meson_content).map_err(|e| e.to_string())?;
  writeln!(
    output,
    "Created root meson.build at {}\n",
    mono_dir.display()
  )
  .ok();
  Ok(())
}
