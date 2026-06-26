use star_setup::{cli::BuildType, config::ConfigEntry};

pub fn sample_entry() -> ConfigEntry {
  ConfigEntry {
    ssh: true,
    build_type: BuildType::Release,
    build_dir: "build".to_string(),
    mono_dir: "mono".to_string(),
    no_build: false,
    clean: true,
    verbose: false,
    timing: false,
    dry_run: false,
    cmake_flags: vec![],
    meson_flags: vec![],
  }
}
