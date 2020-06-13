mod numbers;

use numbers::number_funcs;

fn main() {
    let mut numbers = number_funcs::get_numbers();
    println!("Input numbers are: {:?}", numbers);
    let average = number_funcs::get_average(&numbers);
    println!("Average is {}", average);
    let median = number_funcs::get_median(&mut numbers);
    println!("Median is {}", median);
    let count_tup = number_funcs::get_mode(&numbers);
    if count_tup.1 != -1 {
        println!("Mode is {} for {} times", count_tup.0, count_tup.1);
    }
}
