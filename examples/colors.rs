use std::io::Write;

fn main() {
  let mut diff = ansi_diff::Diff::new(get_size());
  // todo: on sigwinch, diff.resize()
  let start = std::time::Instant::now();
  loop {
    print!["{}", diff.update(&format![
      "{}\n{}\n{}\n",
      "ABCDEFGHIJKLMNOPQRSTUVWXYZ`0123456789-=",
      format![
        "\x1b[31m{:.0}\x1b[39m seconds have elapsed \x1b[32m!!!\x1b[39m",
        start.elapsed().as_secs_f32(),
      ],
      "abcdefghijklmnopqrstuvwxyz~!@#$%^&*()_+",
    ])];
    std::io::stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

fn get_size() -> (u32,u32) {
  term_size::dimensions().map(|(w,h)| (w as u32, h as u32)).unwrap()
}
