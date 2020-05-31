const NUMBERS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const SENTENCES: [&str; 12] = [
    "12 drummers drumming",
    "Eleven pipers piping",
    "Ten lords a leaping",
    "Nine ladies dancing",
    "Eight maids a milking",
    "Seven swans a swimming",
    "Six geese a laying",
    "Five gold rings, badam-pam-pam",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "And a partridge in a pear tree",
];

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas", NUMBERS[i]);
        println!("My true love gave to me");

        for j in (0..i + 1).rev() {
            if i == 0 {
                println!("A partridge in a pear tree");
            } else {
                println!("{}", SENTENCES[11 - j]);
            }
        }

        println!();
    }
}
