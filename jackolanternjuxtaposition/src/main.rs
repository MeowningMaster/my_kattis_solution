use std::io::{BufRead, stdin};

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .nth(0).unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    
    println!("{}", nums.into_iter().product::<i32>());
}
