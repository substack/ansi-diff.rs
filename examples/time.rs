fn main() {
  let mut diff = ansi_diff::Diff::new(get_size());
  // todo: on sigwinch, diff.resize()
  loop {
    print!["{}", diff.update(&format![
      "-------------\nthe time is: {:?}\n-------------\n",
      std::time::Instant::now()
    ])];
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

fn get_size() -> (u32,u32) {
  term_size::dimensions().map(|(w,h)| (w as u32, h as u32)).unwrap()
}
