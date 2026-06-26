use star_setup::cli::BuildType;

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
  use std::str::FromStr;
  assert_eq!(BuildType::from_str("debug").unwrap(), BuildType::Debug);
  assert_eq!(BuildType::from_str("release").unwrap(), BuildType::Release);
  assert_eq!(
    BuildType::from_str("rel-with-deb-info").unwrap(),
    BuildType::RelWithDebInfo
  );
  assert_eq!(
    BuildType::from_str("relwithdebinfo").unwrap(),
    BuildType::RelWithDebInfo
  );
  assert_eq!(
    BuildType::from_str("debugoptimized").unwrap(),
    BuildType::RelWithDebInfo
  );
  assert_eq!(
    BuildType::from_str("min-size-rel").unwrap(),
    BuildType::MinSizeRel
  );
  assert_eq!(
    BuildType::from_str("minsizerel").unwrap(),
    BuildType::MinSizeRel
  );
  assert_eq!(
    BuildType::from_str("minsize").unwrap(),
    BuildType::MinSizeRel
  );
}

#[test]
fn test_from_str_error() {
  use std::str::FromStr;
  assert!(BuildType::from_str("unknown").is_err());
}

#[test]
fn test_build_system_from_str() {
  use star_setup::cli::BuildSystem;
  use std::str::FromStr;
  assert_eq!(BuildSystem::from_str("cmake").unwrap(), BuildSystem::Cmake);
  assert_eq!(BuildSystem::from_str("CMAKE").unwrap(), BuildSystem::Cmake);
  assert_eq!(BuildSystem::from_str("meson").unwrap(), BuildSystem::Meson);
  assert_eq!(BuildSystem::from_str("MESON").unwrap(), BuildSystem::Meson);
  assert!(BuildSystem::from_str("ninja").is_err());
}
