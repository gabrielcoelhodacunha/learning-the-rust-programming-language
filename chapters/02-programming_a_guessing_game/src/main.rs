use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::num::ParseIntError;

fn main() {
    let secret_number = generate_secret_number();

    println!("Welcome to GUESS THE NUMBER!");
    loop {
        let guess: u32 = match read_guess() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn generate_secret_number() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}

fn read_guess() -> Result<u32, ParseIntError> {
    let mut guess = String::new();

    print!("What is your guess? ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess.trim().parse();
}
