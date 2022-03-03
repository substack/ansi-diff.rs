#[test]
fn split() {
  assert_eq![
    ansi_diff::ansi_split("hello cool ok"),
    vec!["hello cool ok"]
  ];
  assert_eq![
    ansi_diff::ansi_split("\x1b[31mhello\x1b[39m cool ok"),
    vec!["", "\x1b[31m", "hello", "\x1b[39m", " cool ok"]
  ];
  assert_eq![
    ansi_diff::ansi_split(
      "\x1b[31m\x1b[1mhello\x1b[22m\x1b[39m \x1b[32mcool ok\x1b[39m\n"
    ),
    vec![
      "",
      "\x1b[31m\x1b[1m",
      "hello",
      "\x1b[22m\x1b[39m",
      " ",
      "\x1b[32m",
      "cool ok",
      "\x1b[39m",
      "\n"
    ]
  ];
}
