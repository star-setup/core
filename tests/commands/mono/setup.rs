use star_setup::commands::build_repo_list;

fn to_string_vec(slice: &[&str]) -> Vec<String> {
  slice.iter().map(ToString::to_string).collect()
}

#[test]
fn test_build_repo_list_test_repo_first() {
  let deps = to_string_vec(&["user/lib1", "user/lib2"]);
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result[0], "user/testrepo");
}

#[test]
fn test_build_repo_list_includes_deps() {
  let deps = to_string_vec(&["user/lib1", "user/lib2"]);
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result.len(), 3);
}

#[test]
fn test_build_repo_list_dedupes_test_repo_in_deps() {
  let deps = to_string_vec(&["user/lib1", "user/testrepo"]);
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result, to_string_vec(&["user/testrepo", "user/lib1"]));
}

#[test]
fn test_build_repo_list_dedupes_duplicate_deps() {
  let deps = to_string_vec(&["user/lib1", "user/lib1"]);
  let result = build_repo_list("user/testrepo", &deps);
  assert_eq!(result, to_string_vec(&["user/testrepo", "user/lib1"]));
}

#[test]
fn test_build_repo_list_no_deps() {
  let result = build_repo_list("user/testrepo", &[]);
  assert_eq!(result, to_string_vec(&["user/testrepo"]));
}
