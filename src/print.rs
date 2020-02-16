pub fn run() {
  // Print to console
  println!("Hello world from print.rs");

  // Basic formatting
  println!("{} is from {}", "Brendon", 1);

  // named arguments
  println!(
    "{name} likes to play {activity}",
    name = "John",
    activity = "piano"
  );

  // Placeholder for debug trait
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "Hello"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}
