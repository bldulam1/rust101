// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growale, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let mut hello = String::from("Hello");

  // Get length
  println!("Length: {}", hello.len());

  // Push string and char
  hello.push_str(" World");
  hello.push('!');
  println!("{}", hello);

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if string is empty
  println!("Is empty {}", hello.is_empty());

  // Check if string contains
  println!("Contains 'World' {}", hello.contains("World"));

  // Replace
  println!("Replace: {}", hello.replace("World", "There"));

  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  println!("{}", hello);

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  assert_eq!(10, s.capacity());

  println!("{}", s)
}
