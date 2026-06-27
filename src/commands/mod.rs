pub mod build;
pub use build::{build_project, cmake_build, meson_build, npm_build};
pub mod header;
pub use header::{print_mode_header, ModeHeader};
pub mod mono;
pub use mono::{
  build_repo_list, create_mono_repo_cmakelists, create_mono_repo_mesonbuild,
  create_mono_repo_package_json, hoist_wraps, mono_repo_mode, resolve_repos_for_mono,
  resolve_test_repo,
  wraps::{parse_project_name, parse_provide_pairs},
};
pub mod single;
pub use single::single_repo_mode;
pub mod setup;
pub use setup::{configure_and_build, extract_repo_input, prepare_build_dir};
