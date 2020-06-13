use std::collections::HashMap;
use std::io;

pub fn get_mode(numbers: &Vec<f64>) -> (f64, i32) {
    let mut map = HashMap::new();

    for number in numbers {
        let counter = map.entry(number.to_string()).or_insert(0i32);
        *counter += 1;
    }

    let mut count_vec: Vec<(&String, &i32)> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    if let Ok(num) = count_vec[0].0.parse() {
        (num, *count_vec[0].1)
    } else {
        (0f64, -1i32)
    }
}

pub fn get_median(numbers: &mut Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pos = numbers.len() / 2;
    numbers[pos]
}

pub fn get_average(numbers: &Vec<f64>) -> f64 {
    let mut acc = 0f64;
    for number in numbers {
        acc += number;
    }

    let result: f64 = acc / (numbers.len() as f64);

    result
}

pub fn get_numbers() -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();
    loop {
        let input = get_input();
        let mut result = true;
        for cell in input.split_whitespace() {
            let value: f64 = match cell.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Input must all be numbers");
                    result = false;
                    break;
                }
            };
            numbers.push(value);
        }

        if result {
            break;
        }

        numbers.clear();
    }

    numbers
}

pub fn get_input() -> String {
    let mut input = String::new();

    println!("Please input some numbers, splited by spaces.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input
}
