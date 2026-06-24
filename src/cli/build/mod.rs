pub mod types;
pub use types::{BuildSystem, BuildType};
pub mod detect;
pub use detect::{detect_build_system, detect_mono_build_system};
