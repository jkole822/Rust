mod logic;

use logic::fibonacci;
use std::{env, io};

fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("Connected to DB at: {}", db_url);
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
