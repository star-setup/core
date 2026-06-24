use star_setup::config::display::format_entry;
use super::fixtures::sample_entry;

#[test]
fn test_format_entry_contains_fields() {
  let entry = sample_entry();
  let output = format_entry(&entry);
  assert!(output.contains("SSH: true"));
  assert!(output.contains("Build Type: Release"));
  assert!(output.contains("Clean flag: true"));
}

#[test]
fn test_format_entry_single_cmake_flag() {
  let mut entry = sample_entry();
  entry.cmake_flags = vec!["-DTEST=ON".to_string()];
  let output = format_entry(&entry);
  assert!(output.contains("CMake argument: -DTEST=ON"));
}

#[test]
fn test_format_entry_multiple_cmake_flags() {
  let mut entry = sample_entry();
  entry.cmake_flags = vec!["-DTEST=ON".to_string(), "-DDEBUG=OFF".to_string()];
  let output = format_entry(&entry);
  assert!(output.contains("CMake arguments:"));
  assert!(output.contains("-DTEST=ON"));
  assert!(output.contains("-DDEBUG=OFF"));
}
