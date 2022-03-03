use std::io::Write;

fn main() {
  let mut diff = ansi_diff::Diff::new(get_size());
  // todo: on sigwinch, diff.resize()
  let start = std::time::Instant::now();
  loop {
    print!["{}", diff.update(&format![
      "-------------\nseconds elapsed: {:.0} ...\n-------------\n",
      start.elapsed().as_secs_f32()
    ])];
    std::io::stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

fn get_size() -> (u32,u32) {
  term_size::dimensions().map(|(w,h)| (w as u32, h as u32)).unwrap()
}
