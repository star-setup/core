use star_setup::cli::{BuildSystem, BuildType};
use std::str::FromStr;

#[test]
fn test_to_cmake_all_variants() {
  assert_eq!(BuildType::Debug.to_cmake(), "Debug");
  assert_eq!(BuildType::Release.to_cmake(), "Release");
  assert_eq!(BuildType::RelWithDebInfo.to_cmake(), "RelWithDebInfo");
  assert_eq!(BuildType::MinSizeRel.to_cmake(), "MinSizeRel");
}

#[test]
fn test_to_meson_all_variants() {
  assert_eq!(BuildType::Debug.to_meson(), "debug");
  assert_eq!(BuildType::Release.to_meson(), "release");
  assert_eq!(BuildType::RelWithDebInfo.to_meson(), "debugoptimized");
  assert_eq!(BuildType::MinSizeRel.to_meson(), "minsize");
}

#[test]
fn test_from_str_all_variants() {
  let cases = [
    ("debug", BuildType::Debug),
    ("release", BuildType::Release),
    ("rel-with-deb-info", BuildType::RelWithDebInfo),
    ("relwithdebinfo", BuildType::RelWithDebInfo),
    ("debugoptimized", BuildType::RelWithDebInfo),
    ("min-size-rel", BuildType::MinSizeRel),
    ("minsizerel", BuildType::MinSizeRel),
    ("minsize", BuildType::MinSizeRel),
  ];

  for (input, expected) in cases {
    assert_eq!(
      BuildType::from_str(input).unwrap(),
      expected,
      "Failed on input: {input}"
    );
  }
}

#[test]
fn test_from_str_error() {
  assert!(BuildType::from_str("unknown").is_err());
}

#[test]
fn test_build_system_from_str() {
  let success_cases = [
    ("cmake", BuildSystem::Cmake),
    ("CMAKE", BuildSystem::Cmake),
    ("meson", BuildSystem::Meson),
    ("MESON", BuildSystem::Meson),
  ];

  for (input, expected) in success_cases {
    assert_eq!(
      BuildSystem::from_str(input).unwrap(),
      expected,
      "Failed on valid input: {input}"
    );
  }

  assert!(
    BuildSystem::from_str("ninja").is_err(),
    "Expected error for invalid input: ninja"
  );
}
