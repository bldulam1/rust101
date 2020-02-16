// Tuples group together values of different types
// Max 12 elements

fn run() {
  let person: (&str, &str, i8) = ("Brendon", "Yokohama", 28);
  println!("{} is from {} and is {}", person.0, person.1, person.2);
}

pub fn main() {
  run();
}
