use std::io::{stdin, BufRead};

fn main() {
    let n = read_num();
    let bin = format!("{:b}", n);
    let bin= bin.chars().rev().collect::<String>();
    let n = i32::from_str_radix(bin.as_str(), 2).unwrap();

    println!("{}", n);
}

fn read_num() -> i32 {
    stdin()
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .trim_end()
        .parse()
        .unwrap()
}
