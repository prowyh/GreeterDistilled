use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("Guess number!");
    print!("Please give your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(& mut guess).expect("Failed to read line.");
    println!("Your guess: {}", guess);
}

