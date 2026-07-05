pub mod build;
pub use build::meson_build;
pub mod config;
pub use config::create_mono_repo_mesonbuild;
pub mod wraps;
pub use wraps::{hoist_wraps, parse_project_name, parse_provide_pairs};
