// to use this example, enable iterator and extended-siginfo features in Cargo.toml dependencies:
// signal-hook = { version = "0.3.13", features = [ "iterator", "extended-siginfo" ] }

use std::io::Write;
use signal_hook::{iterator::{SignalsInfo,exfiltrator::WithOrigin},consts::signal::SIGWINCH};
use std::sync::{Arc,Mutex};
use ansi_diff::Diff;

fn main() {
  let diff = Arc::new(Mutex::new(Diff::new(get_size())));
  let diff_c = diff.clone();
  std::thread::spawn(move || { resizer(diff_c) });

  let start = std::time::Instant::now();
  loop {
    print!["{}", diff.lock().unwrap().update(&format![
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

fn resizer(diff: Arc<Mutex<Diff>>) {
  let mut signals = SignalsInfo::<WithOrigin>::new(&vec![SIGWINCH]).unwrap();
  for info in &mut signals {
    if info.signal == SIGWINCH { diff.lock().unwrap().resize(get_size()) }
  }
}
