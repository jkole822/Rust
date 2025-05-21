mod logic;

use logic::fibonacci;
use std::{env, io};

fn main() {
    let greeting = env::var("FIB_GREETING").unwrap_or("No greeting found".to_string());
    println!("Greeting from secret: {}", greeting);
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
