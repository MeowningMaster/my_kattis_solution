use std::io::{BufRead, stdin};

fn main() {
    let sample = vec![1, 1, 2, 2, 2, 8];
    let nums: Vec<i32> =
        stdin().lock().lines().nth(0).unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 0..sample.len() {
        print!("{} ", sample[i] - nums[i]);
    }
}
