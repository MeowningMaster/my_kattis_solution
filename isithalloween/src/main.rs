use std::io::{stdin, BufRead};

fn main() {
    let line = stdin().lock().lines().nth(0).unwrap().unwrap();
    let mut parts = line.split_whitespace();

    let month = parts.next().unwrap();
    let day = parts.next().unwrap().parse::<i32>().unwrap();

    let message = match (month, day) {
        ("OCT", 31) | ("DEC", 25) => "yup",
        _ => "nope"
    };

    println!("{}", message);
}
