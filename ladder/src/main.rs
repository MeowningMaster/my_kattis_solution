use std::io::{BufRead, stdin};

fn main() {
    let n: Vec<i32> = stdin()
    .lock()
    .lines()
    .nth(0).unwrap().unwrap()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();

    let (h, v) = (n[0], n[1]);
    let r = ((h as f64) / (v as f64).to_radians().sin()).ceil();
    println!("{}", r as i32);
}