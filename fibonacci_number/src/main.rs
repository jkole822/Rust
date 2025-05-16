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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), 0);
    }

    #[test]
    fn test_fibonacci_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_five() {
        assert_eq!(fibonacci(5), 3);
    }

    #[test]
    fn test_fibonacci_seven() {
        assert_eq!(fibonacci(7), 8);
    }
}
