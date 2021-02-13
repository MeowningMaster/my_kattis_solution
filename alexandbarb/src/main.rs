use std::io;

use io::BufRead;

fn main() {
    let nums: Vec<i32> =
        io::stdin().lock().lines().nth(0).unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    let (k, m, n) = (nums[0], nums[1], nums[2]);
    let a = vec!(true; k as usize  + 1);

    for i in 0..=k {
        // TODO
    }
}
