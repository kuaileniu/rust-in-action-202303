fn main() {
  let a: f32 = 42.24;
  let franken_type: u32 = unsafe { std::mem::transmute(a) };

  println!("{}", franken_type);
  println!("{:032b}", franken_type);

  let b: f32 = unsafe { std::mem::transmute(franken_type) };
  println!("{}", b);
  // println!("{:032b}", b); // the trait `Binary` is not implemented for `f32`
  println!("Hello, world!");
}
