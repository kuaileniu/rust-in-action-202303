use std::fmt::{Display, Result};

#[derive(Debug)]
enum FileState {
  Open,
  Closed,
}
#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl File {
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::Closed,
    }
  }
}

impl Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
    write!(f, "{} ({})", self.name, self.state)
  }
}

impl Display for FileState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSE"),
    }
  }
}
fn main() {
  let f6 = File::new("f6.txt");
  println!("{:?}", f6);
  println!("{}", f6);
  println!("Hello, world!");
}
