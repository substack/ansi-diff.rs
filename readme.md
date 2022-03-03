# ansi-diff

diff successive buffers with embedded ansi codes in rust, outputting a minimal change

You can use this crate to build command-line interfaces in an "immediate mode" way where you can
produce pages of output which will be diffed to avoid the flicker you would otherwise get with that
otherwise convenient approach from clearing the whole screen between updates.

You can include ansi codes for formatting your output but codes that adjust the cursor position are
not (yet) supported. You can manage cursor positions to place panes of content using the output of
this crate, but you will need to write that glue yourself.

This package is a rust port of the node.js [ansi-diff][] package also including code adapted from
[ansi-split][] and [ansi-regex][].

[ansi-diff]: https://github.com/mafintosh/ansi-diff
[ansi-split]: https://github.com/mafintosh/ansi-split
[ansi-regex]: https://github.com/chalk/ansi-regex

## example: time

This is a simple example that prints 4 lines of text and updates a counter for the seconds elapsed:

``` rs
use std::io::Write;

fn main() {
  let mut diff = ansi_diff::Diff::new(get_size());
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
```

## example: resize

This is a version of the previous time example that sets up terminal resize hooks for the SIGWINCH
event: 

``` rs
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
```

## example: colors

This is a more involved version of the first example that includes ansi formatting in the output
stream:

``` rs
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
```

# license

bsd-2-clause
