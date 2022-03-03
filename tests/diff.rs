#[test]
fn diff() {
  let mut diff = ansi_diff::Diff::new((80,24));
  assert_eq![
    diff.update("ABC\nwhatever x=0\nDEF\n"),
    "ABC\nwhatever x=0\nDEF\n"
  ];
  assert_eq![
    diff.update("ABC\nwhatever x=1\nDEF\n"),
    "\x1b[11C\x1b[2A1\x1b[12D\x1b[2B"
  ];
  assert_eq![
    diff.update("ABC\nwhatever x=2\nDEF\n"),
    "\x1b[11C\x1b[2A2\x1b[12D\x1b[2B"
  ];
  assert_eq![
    diff.update("ABC\nwhatever x=3\nDEF\n"),
    "\x1b[11C\x1b[2A3\x1b[12D\x1b[2B"
  ];
  assert_eq![
    diff.update("ABC\nwhatever x=4\nDEF\n"),
    "\x1b[11C\x1b[2A4\x1b[12D\x1b[2B"
  ];
  assert_eq![
    diff.update("abc\nwhatever x=5\nDEF\n"),
    "\x1b[3Aabc\n\x1b[11C5\x1b[12D\x1b[2B"
  ];
}
