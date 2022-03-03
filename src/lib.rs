use lazy_static::lazy_static as lstatic;
type P = u32;

const CLEAR_LINE: [u32;4] = [0x1b, 0x5b, 0x30, 0x4b];
const NEWLINE: [u32;1] = [0x0a];

pub fn ansi_split(input: &str) -> Vec<String> {
  lstatic! {
    static ref IS_ANSI: regex::Regex = regex::Regex::new(
      r#"(?x)
        [\u001b\u009b][\[\]()\#;]*(?:(?:(?:[A-Za-z\d]*(?:;[A-Za-z\d]*)*)?\u0007)
        | (?:(?:\d{1,4}(?:;\d{0,4})*)?[\dA-PRZcf-ntqry=><~]))
      "#,
    ).unwrap();
  };
  let mut ptr = 0;
  let mut result = vec![];
  let ibytes = input.bytes().collect::<Vec<u8>>();
  for c in IS_ANSI.captures_iter(input) {
    let m = c.get(0).unwrap();
    let part = m.as_str().to_string();
    let offset = m.start();
    if ptr != offset && ptr < offset {
      result.push(String::from_utf8(ibytes[ptr..offset].to_vec()).unwrap());
    }
    if ptr == offset && !result.is_empty() {
      result.last_mut().unwrap().push_str(&part);
    } else {
      if offset == 0 { result.push(String::default()) }
      result.push(part);
    }
    ptr = m.end();
  }
  if ptr < ibytes.len() {
    result.push(String::from_utf8(ibytes[ptr..].to_vec()).unwrap());
  }
  if result.is_empty() { return vec![input.to_string()] }
  result
}

pub struct Diff {
  x: P,
  y: P,
  width: P,
  height: P,
  buffer: String,
  out: Vec<String>,
  lines: Vec<Line>,
}

impl Diff {
  pub fn new(size: (P,P)) -> Self {
    Diff {
      x: 0,
      y: 0,
      width: size.0,
      height: size.1,
      buffer: String::default(),
      out: vec![],
      lines: vec![],
    }
  }
  pub fn resize(&mut self, width: P, height: P) {
    self.width = width;
    self.height = height;
    let buf = self.buffer.clone();
    self.update(&buf);
    match self.lines.last() {
      Some(last) => {
        self.x = last.remainder;
        self.y = last.y + last.height;
      },
      None => {
        self.x = 0;
        self.y = 0;
      }
    }
  }
  pub fn update(&mut self, buffer: &str) -> String {
    self.buffer = buffer.to_string();
    let next_lines = Line::split(buffer, self.width);
    let min = next_lines.len().min(self.lines.len());
    self.out = vec![];
    let mut scrub = false;
    for i in 0..min {
      let a = next_lines.get(i).unwrap();
      let (b_y, b_length, b_height) = {
        let b = self.lines.get(i).unwrap();
        if a == b { continue }
        if !scrub && self.x != self.width && Self::inline_diff(&a,&b) {
          let left = a.diff_left(b) as usize;
          let right = a.diff_right(b) as usize;
          let slice = &a.raw[left .. (a.raw.len() - right).max(left)];
          if left + right > 4 && left + slice.len() < self.width as usize - 1 {
            self.move_to(left as P, a.y);
            self.push(&String::from_iter(slice));
            self.x += slice.len() as P;
            continue
          }
        }
        (b.y, b.length, b.height)
      };
      self.move_to(0, a.y);
      self.write(a);
      if a.y != b_y || a.height != b_height { scrub = true }
      if b_length > a.length || scrub { self.push(&to_str(CLEAR_LINE)) }
      if a.newline { self.newline() }
    }

    for line in &next_lines[min..] {
      self.move_to(0, line.y);
      self.write(line);
      if scrub { self.push(&to_str(CLEAR_LINE)) }
      if line.newline { self.newline() }
    }

    let prev_last = self.lines.last();
    let next_last = next_lines.last();
    let clear = match (prev_last, next_last) {
      (Some(plast),None) => Some(plast.y + plast.height),
      (Some(plast),Some(nlast)) => {
        if nlast.y + nlast.height < plast.y + plast.height {
          Some(plast.y + plast.height)
        } else {
          None
        }
      },
      (None,_) => None,
    };
    if let Some(n) = clear {
      self.clear_down(n);
    }

    // todo: opts.move_to
    if let Some(last) = next_last {
      self.move_to(last.remainder, last.y + last.height);
    }

    self.lines = next_lines;
    self.out.join("")
  }
  fn inline_diff(a: &Line, b: &Line) -> bool {
    a.length == b.length
      && a.parts.len() == 1 && b.parts.len() == 1
      && a.y == b.y
      && a.newline && b.newline
      && a.width == b.width
  }
  fn move_to(&mut self, x: P, y: P) {
    if x > self.x { self.push(&Self::move_right(x - self.x)) }
    else if x < self.x { self.push(&Self::move_left(self.x - x)) }
    if y > self.y { self.push(&Self::move_down(y - self.y)) }
    else if y < self.y { self.push(&Self::move_up(self.y - y)) }
    self.x = x;
    self.y = y;
  }
  fn push(&mut self, buf: &str) {
    self.out.push(buf.to_string());
  }
  fn newline(&mut self) {
    self.push(&to_str(NEWLINE));
    self.x = 0;
    self.y += 1;
  }
  fn clear_down(&mut self, y: P) {
    let mut x = self.x;
    let mut i = self.y;
    while i <= y {
      self.move_to(x, i);
      self.push(&to_str(CLEAR_LINE));
      x = 0;
      i += 1;
    }
  }
  fn move_up(n: P) -> String { code1(&[0x1b, 0x5b], n, &[0x41]) }
  fn move_down(n: P) -> String { code1(&[0x1b, 0x5b], n, &[0x42]) }
  fn move_right(n: P) -> String { code1(&[0x1b, 0x5b], n, &[0x43]) }
  fn move_left(n: P) -> String { code1(&[0x1b, 0x5b], n, &[0x44]) }
  fn write(&mut self, line: &Line) {
    self.out.push(line.to_string());
    self.x = line.remainder;
    self.y += line.height;
  }
}

impl ToString for Diff {
  fn to_string(&self) -> String {
    self.buffer.clone()
  }
}

#[derive(Clone,Debug)]
pub struct Line {
  y: P,
  width: P,
  height: P,
  parts: Vec<String>,
  length: usize,
  raw: Vec<char>,
  newline: bool,
  remainder: P,
}

impl Line {
  pub fn new(s: &str, y: P, nl: bool, term_width: P) -> Self {
    let parts = ansi_split(s);
    let length = Self::parts_len(&parts);
    let mut height = length as P / term_width;
    let mut remainder = length as P - (height * term_width);
    if height > 0 && remainder == 0 {
      height -= 1;
      remainder = term_width;
    }
    Self {
      y,
      width: term_width,
      parts,
      length,
      raw: s.chars().collect(),
      newline: nl,
      height,
      remainder,
    }
  }
  pub fn diff_left(&self, other: &Self) -> P {
    let mut left = 0_usize;
    while left < self.length {
      if self.raw.get(left) != other.raw.get(left) { break }
      left += 1;
    }
    left as P
  }
  pub fn diff_right(&self, other: &Self) -> P {
    let mut right = 0;
    while right < self.length {
      let r = self.length - right - 1;
      if self.raw.get(r) != other.raw.get(r) { break }
      right += 1;
    }
    right as P
  }
  pub fn split(input: &str, term_width: P) -> Vec<Self> {
    let mut y = 0;
    let lines = input.split('\n').collect::<Vec<&str>>();
    let len = lines.len();
    lines.iter().enumerate().map(move |(i,line)| {
      let line = Line::new(line, y, i < len - 1, term_width);
      y += line.height + (if line.newline { 1 } else { 0 });
      line
    }).collect()
  }
  fn parts_len(parts: &[String]) -> usize {
    let mut sum = 0;
    let mut i = 0;
    while i < parts.len() {
      sum += parts.get(i).unwrap().len();
      i += 2;
    }
    sum
  }
}

impl ToString for Line {
  fn to_string(&self) -> String {
    String::from_iter(self.raw.iter())
  }
}

impl PartialEq for Line {
  fn eq(&self, other: &Self) -> bool {
    self.y == other.y && self.width == other.width
      && self.raw == other.raw && self.newline == other.newline
  }
}

fn to_str<const N: usize>(xs: [u32;N]) -> String {
  let chars = xs.iter().map(|c| char::from_u32(*c).unwrap()).collect::<Vec<char>>();
  String::from_iter(&chars)
}

fn code1(pre: &[u32], n: P, post: &[u32]) -> String {
  let sn = format!["{}", n];
  let spre = String::from_iter(pre.iter().map(|c| char::from_u32(*c).unwrap()));
  let spost = String::from_iter(post.iter().map(|c| char::from_u32(*c).unwrap()));
  spre + &sn + &spost
}
