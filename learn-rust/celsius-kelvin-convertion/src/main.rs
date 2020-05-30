use std::io;
use std::string::String;

fn main() {
    let mut mode = String::new();

    loop {
        println!("Please input the type convert from, only C or F allowed.");

        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read input.");

        match mode.trim() {
            "C" => {
                println!("Mode C to F");
                let value = get_value();
                let result = celsius_to_kelvin(value);
                println!("Kelvin value is {:.2}.", result);
                break;
            }
            "F" => {
                println!("Mode F to C");
                let value = get_value();
                let result = kelvin_to_celsius(value);
                println!("Celsius value is {:.2}.", result);
                break;
            }
            _ => {
                println!("Must be C or F, input is {}. Please try again.", mode);
                mode = String::new();
            }
        }
    }
}

fn get_value() -> f64{
    let mut value_string = String::new();

    loop {
        println!("Please enter an value.");
        io::stdin()
            .read_line(&mut value_string)
            .expect("Failed to read input.");

        let value:f64 = match value_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be an number, but currently is {}, please try again.", value_string.trim());
                value_string = String::new();
                continue;
            },
        };
        return value;
    }
}

fn celsius_to_kelvin(value: f64) -> f64{
    9.0 * value / 5.0 + 32.0
}

fn kelvin_to_celsius(value: f64) -> f64{
    5.0 / 9.0 * (value - 32.0)
}
