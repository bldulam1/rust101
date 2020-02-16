/**
 * Primitive types
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 *
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 */

pub fn run() {
  // Default is i32
  let _x = 1;

  // Default is f64
  let _y = 2.5;

  // Explicity type
  let _z: i64 = 31416;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;
  println!("{:?}", (_x, _y, _z, is_active));

  // Characters
  let a1 = 'a';
  let face = '\u{1F600}';
  println!("{:?}", (1, 2, 3, 45, 5 > 2, a1, face));

   
}
