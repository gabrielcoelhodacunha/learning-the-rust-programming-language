use crate::utils::{clear_screen, print_and_flush, read_string, wait_for_enter};

pub fn program() {
    loop {
        let mut n = String::new();
        clear_screen();
        print_and_flush("Type a numeric value to calculate it's Fibonacci: ");
        read_string(&mut n);
        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!(
            "The {n}th value in the Fibonacci sequence is {}",
            calculate(n)
        );
        wait_for_enter();
    }
}

fn calculate(n: u128) -> u128 {
    let [mut first, mut second]: [u128; 2] = [0, 1];
    for _ in 0..n {
        [first, second] = [first + second, first];
    }
    first
}
