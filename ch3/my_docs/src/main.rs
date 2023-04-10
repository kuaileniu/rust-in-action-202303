//! Simulating files one step at a time.     // <1>

/// Represents a "file",
/// which probably lives on a file system.   // <2>
#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
}

/// ``` rust
/// # 这里是一级标题
/// ## 这里是二级标题
///  let a: usize=10
/// ```
/// 
impl File {
  /// New files are assumed to be empty, but a name is required.
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
    }
  }

  /// Returns the file's length in bytes.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

fn main() {
  let f1 = File::new("f1.txt");

  let f1_name = f1.name();
  let f1_length = f1.len();

  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}
