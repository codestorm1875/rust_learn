use std::io;

fn main() {
    loop {
        println!("Temperature Converter");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Kelvin to Celsius");
        println!("4. Celsius to Kelvin");
        println!("5. Fahrenheit to Kelvin");
        println!("6. Kelvin to Fahrenheit");
        println!("0. Exit");
        println!("Enter your choice:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue; 
            }
        };

        if choice == 0 {
            println!("Goodbye!");
            break;
        }

        println!("Enter temperature value:");
        let mut temp_input = String::new();
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read line");

        let temp: f64 = match temp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature value");
                continue;
            }
        };

        let result = match choice {
            1 => (temp - 32.0) * 5.0 / 9.0,
            2 => temp * 9.0 / 5.0 + 32.0,
            3 => temp - 273.15,
            4 => temp + 273.15,
            5 => (temp - 32.0) * 5.0 / 9.0 + 273.15,
            6 => (temp - 273.15) * 9.0 / 5.0 + 32.0,
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        println!("Result: {:.2}°", result);
    }
}