mod logic;

use logic::fibonacci;
use std::io;

fn main() {
    println!("Provide a number to get the fibonacci number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };

    println!("The fibonacci number is {}", fibonacci(number));
}
