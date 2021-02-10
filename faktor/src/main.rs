use std::io;

use io::{BufRead};

fn main() {
    let buf: Vec<i32> =
        io::stdin().lock().lines().nth(0).unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap())
        .collect();

    let (a, i) = (buf[0], buf[1]);
    println!("{}", a * (i - 1) + 1);
}