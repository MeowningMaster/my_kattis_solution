use std::io::{stdin, BufRead};

fn main() {
    let mut nums = stdin()
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    nums.sort();
    println!("{}", nums[0] * nums[2]);
}
