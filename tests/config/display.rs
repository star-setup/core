use star_setup::{
  build::BuildType,
  config::{format_entry, ConfigEntry},
};

#[test]
fn test_format_entry_contains_fields() {
  let entry = ConfigEntry {
    ssh: true,
    build_type: BuildType::Release,
    clean: true,
    ..ConfigEntry::default()
  };
  let output = format_entry(&entry);
  assert!(output.contains("SSH: true"));
  assert!(output.contains("Build Type: Release"));
  assert!(output.contains("Clean flag: true"));
  assert!(output.contains("Timing flag: false"));
}

#[test]
fn test_format_entry_single_cmake_flag() {
  let entry = ConfigEntry {
    cmake_flags: vec!["-DTEST=ON".to_string()],
    ..ConfigEntry::default()
  };
  let output = format_entry(&entry);
  assert!(output.contains("CMake argument: -DTEST=ON"));
}

#[test]
fn test_format_entry_multiple_cmake_flags() {
  let entry = ConfigEntry {
    cmake_flags: vec!["-DTEST=ON".to_string(), "-DDEBUG=OFF".to_string()],
    ..ConfigEntry::default()
  };
  let output = format_entry(&entry);
  assert!(output.contains("CMake arguments:"));
  assert!(output.contains("-DTEST=ON"));
  assert!(output.contains("-DDEBUG=OFF"));
}

#[test]
fn test_format_entry_single_meson_flag() {
  let entry = ConfigEntry {
    meson_flags: vec!["-Db_lto=true".to_string()],
    ..ConfigEntry::default()
  };
  let output = format_entry(&entry);
  assert!(output.contains("Meson argument: -Db_lto=true"));
}

#[test]
fn test_format_entry_multiple_meson_flags() {
  let entry = ConfigEntry {
    meson_flags: vec!["-Db_lto=true".to_string(), "-Db_ndebug=true".to_string()],
    ..ConfigEntry::default()
  };
  let output = format_entry(&entry);
  assert!(output.contains("Meson arguments:"));
  assert!(output.contains("-Db_lto=true"));
  assert!(output.contains("-Db_ndebug=true"));
}
