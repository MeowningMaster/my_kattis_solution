use std::io;

use io::BufRead;

fn main() {
    let line = io::stdin()
    .lock()
    .lines().nth(0).unwrap().unwrap();

    let mut c = 0;
    for part in line.split(';') {
        let nums: Vec<i32> = part
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();

        match nums.len() {
            1 => c += 1,
            2 => c += nums[1] - nums[0] + 1,
            _ => panic!("irr")
        }
    }

    println!("{}", c);
}
