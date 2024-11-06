use std::num::ParseIntError;

mod fahrenheit_to_celsius;
mod fibonacci;
mod the_twelve_days_of_christmas;
mod utils;

use utils::{clear_screen, print_and_flush, read_string};

fn main() {
    loop {
        clear_screen();
        let choice = match read_choice() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            0 => break,
            1 => fahrenheit_to_celsius::program(),
            2 => fibonacci::program(),
            3 => the_twelve_days_of_christmas::program(),
            _ => continue,
        }
    }
}

fn read_choice() -> Result<u8, ParseIntError> {
    let mut choice = String::new();

    println!("Menu");
    println!("[0] - Exit");
    println!("[1] - Convert Fahrenheit to Celsius");
    println!("[2] - Calculate nth Fibonacci");
    println!("[3] - Print the lyrics to the Christmas carol 'The Twelve Days of Christmas'");
    print_and_flush("Type the number between [] to proceed: ");
    read_string(&mut choice);

    choice.trim().parse()
}
