use star_setup::{cli::BuildType, config::types::ConfigEntry};

pub fn sample_entry() -> ConfigEntry {
  ConfigEntry {
    ssh: true,
    build_type: BuildType::Release,
    build_dir: "build".to_string(),
    mono_dir: "mono".to_string(),
    no_build: false,
    clean: true,
    verbose: false,
    cmake_flags: vec![],
  }
}
