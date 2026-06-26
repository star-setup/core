use star_setup::commands::build_repo_list;

#[test]
fn test_build_repo_list_test_repo_first() {
  let deps = vec!["user/lib1".to_string(), "user/lib2".to_string()];
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result[0], "user/testrepo");
}

#[test]
fn test_build_repo_list_includes_deps() {
  let deps = vec!["user/lib1".to_string(), "user/lib2".to_string()];
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result.len(), 3);
}

#[test]
fn test_build_repo_list_dedupes_test_repo_in_deps() {
  let deps = vec!["user/lib1".to_string(), "user/testrepo".to_string()];
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result.len(), 2);
  assert_eq!(result[0], "user/testrepo");
}

#[test]
fn test_build_repo_list_dedupes_duplicate_deps() {
  let deps = vec!["user/lib1".to_string(), "user/lib1".to_string()];
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result.len(), 2);
}

#[test]
fn test_build_repo_list_no_deps() {
  let result = build_repo_list("user/testrepo", &[]);
  assert_eq!(result, vec!["user/testrepo"]);
}
