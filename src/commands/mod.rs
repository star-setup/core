pub mod build;
pub mod header;
pub mod mono;
pub use mono::{
  build_repo_list, create_mono_repo_cmakelists, create_mono_repo_mesonbuild, hoist_wraps,
  mono_repo_mode, resolve_repos_for_mono, resolve_test_repo,
  wraps::{parse_project_name, parse_provide_pairs},
};
pub mod single;
pub use single::single_repo_mode;
