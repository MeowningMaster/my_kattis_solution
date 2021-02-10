use std::{cmp::max, io};
use io::{BufRead, stdin};

fn main() {
    let buf: Vec<i32> = 
        stdin().lock().lines().nth(0).unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.chars().rev().collect::<String>().parse().unwrap())
        .collect();
    
    println!("{}", max(buf[0], buf[1]));
}
