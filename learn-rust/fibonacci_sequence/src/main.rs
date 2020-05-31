use std::io;
use std::string::String;

fn main() {
    let mut input = String::new();
    let mut count:u32;

    loop {
        println!("Enter the count of sequence to generate. Must between 1 to 47.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        count = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid input, please try again.");
                input = String::new();
                continue;
            }
        };

        if count > 0 && count <= 47{
            break;
        }

        println!("Out of range. Please try again.");
        input = String::new();
    }

    print_sequence(count);
}

fn print_sequence(mut count: u32){
    if count == 1 {
        println!("0");
        return;
    }

    if count == 2 {
        println!("0 1");
        return;
    }

    print!("0 1");
    count = count - 2;
    let mut seq = [0u32, 1, 1];
    while count != 0 {
        print!(" {}", seq[2]);
        seq[0] = seq[1];
        seq[1] = seq[2];
        seq[2] = seq[0] + seq[1];
        count = count - 1;
    }
    println!();
}
