fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_pos = 0;
    let mut i = 0;

    while i < list.len() {
        if list[i] > list[largest_pos] {
            largest_pos = i;
        }
        i += 1;
    }

    &list[largest_pos]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
