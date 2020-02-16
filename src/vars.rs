// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a data-scoped language

pub fn run() {
  let name = "Luke";
  let mut age = 2;

  println!("My name is {name} and I am {age}", name = name, age = age);
  age += 1;
  println!("My name is {name} and I am {age}", name = name, age = age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brendon", 28);

  println!("{} is {}", my_name, my_age)
}
