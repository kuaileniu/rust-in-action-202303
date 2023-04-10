#[allow(arithmetic_overflow)]
fn main() {
  let (a, b) = (200, 100);
  let c: u8 = a + b; // attempt to compute `200_u8 + 100_u8`, which would overflow
  println!("200 + 100 = {}", c); // 因为抛出 panic，无法执行完毕。
  // 若定要编译执行成功需：
  // rustc -O main.rs  && ./main
}
