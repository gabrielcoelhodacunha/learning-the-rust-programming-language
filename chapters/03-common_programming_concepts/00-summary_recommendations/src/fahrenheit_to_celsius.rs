use crate::utils::{clear_screen, print_and_flush, read_string, wait_for_enter};

pub fn program() {
    loop {
        let mut fahrenheit = String::new();
        clear_screen();
        print_and_flush(
            "Type a numeric value to convert to Celsius or any other value to return to the menu: ",
        );
        read_string(&mut fahrenheit);
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!("{fahrenheit}°F = {:.2}°C", convert(fahrenheit));
        wait_for_enter();
    }
}

fn convert(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
