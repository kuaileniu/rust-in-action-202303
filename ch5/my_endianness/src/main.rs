use std::mem::transmute;

fn main() {
  let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
  let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xaa];
  // let a_16: i16 = unsafe { transmute(big_endian) };
  // let b_16: i16 = unsafe { transmute(little_endian) };
  let a: i32 = unsafe { transmute(big_endian) };
  let b: i32 = unsafe { transmute(little_endian) };

  println!("{} vs {}", a, b);
  // println!("{:016b},{:016b}", a_16, b_16);
  println!("{:032b},{:032b}", a, b);
}
