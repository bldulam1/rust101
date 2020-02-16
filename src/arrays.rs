// Arrays - fixed list where elements are the same data types
use std::mem;

fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Get single value
  println!("{}", numbers[1]);

  // Get array length
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);
}

pub fn main() {
  run();
}
