use std::io;

fn temperature_conversion(unit: &str, number: f32) -> f32 {
    if unit == "C" {
        return number * 9.0 / 5.0 + 32.0
    }

    (number - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut initial_unit = String::new();
    let mut trimmed_initial_unit: &str;
    let final_unit: &str;

    loop {
        println!("Provide indicate whether you want to convert Celsius to Fahrenheit (C) or Fahrenheit to Celsius (F):");

        initial_unit.clear();

        io::stdin().read_line(&mut initial_unit).expect("Failed to read line");

        trimmed_initial_unit = initial_unit.trim();

        match trimmed_initial_unit {
            "C" => {
                final_unit = "F";
                break;
            },
            "F" => {
                final_unit = "C";
                break;
            },
            _ => println!("Invalid input. Please enter 'C' or 'F'."),
        };
    }

    let number: f32;

    loop {
        println!("Enter the temperature to convert:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f32>() {
            Ok(num) => {
                number = num;
                break;
            },
            Err(_) => println!("Not a valid number. Try again."),
        };
    }

    println!(
        "The temperature in {} is {} degrees {}.",
        trimmed_initial_unit,
        temperature_conversion(&trimmed_initial_unit, number),
        final_unit
    );
}
