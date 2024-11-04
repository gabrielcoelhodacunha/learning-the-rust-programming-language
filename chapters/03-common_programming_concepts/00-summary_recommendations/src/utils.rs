use std::io::{self, Read, Write};

pub fn clear_screen() {
    clearscreen::clear().unwrap();
}

pub fn read_string(value: &mut String) {
    io::stdin().read_line(value).unwrap();
}

pub fn print_and_flush(value: &str) {
    print!("{value}");
    io::stdout().flush().unwrap();
}

pub fn wait_for_enter() {
    print_and_flush("Press ENTER to continue ...");
    io::stdin().read(&mut [0]).unwrap();
}
