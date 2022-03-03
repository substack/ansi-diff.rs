#[test]
fn split() {
  assert_eq![
   ansi_diff::ansi_split("hello cool ok"),
   vec!["hello cool ok"]
  ];
}
