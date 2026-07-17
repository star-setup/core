pub mod types;
pub use types::{BuildSystem, BuildType};
pub mod detect;
pub use detect::{detect_build_system, detect_mono_build_system};
pub mod dispatch;
pub use dispatch::{build_project, generate_mono_config};
pub mod common;

pub mod cmake;
pub use cmake::{cmake_build, create_mono_repo_cmakelists};
pub mod meson;
pub use meson::{
  create_mono_repo_mesonbuild, hoist_wraps, meson_build, parse_project_name, parse_provide_pairs,
};
pub mod npm;
pub use npm::{
  create_mono_repo_package_json, generate_dev_scripts, generate_terminal_scripts,
  generate_watch_scripts, maybe_open_dev_server, npm_build, open_dev_server, open_scripts,
  read_package_json, resolve_dev_command,
};
