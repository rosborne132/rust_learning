// Import input / Output library that comes standard from Rust.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  // Generates a number between 1 to 100.
  let secret_number = rand::thread_rng().gen_range(1..11);

  loop {
    println!("Please input your guess.");

    // The 'let' keyword means we are creating a new variable.
    // The 'mut' keyword means that the variable is mutable.
    let mut guess = String::new();

    // Takes the users input and appends it to the string variable 'guess'.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Use case for shadowing. Shadowing also us to overwrite the original value.
    let guess: u32 = match guess.trim().parse() {
      Ok(value) => value,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
