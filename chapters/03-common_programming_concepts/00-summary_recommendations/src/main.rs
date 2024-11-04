use std::num::ParseIntError;

mod fahrenheit_to_celsius;
mod utils;

use utils::{clear_screen, read_string};

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
            /* Todo
            2 => fibonacci::program(),
            3 => the_twelve_days_of_christmas::program(),
            */
            _ => continue,
        }
    }
}

fn read_choice() -> Result<u8, ParseIntError> {
    let mut choice = String::new();

    println!("Type the number between [] to proceed.");
    println!("[0] - Exit");
    println!("[1] - Convert Fahrenheit to Celsius");
    read_string(&mut choice);

    choice.trim().parse()
}
