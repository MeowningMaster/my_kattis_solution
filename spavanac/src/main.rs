use std::io::{BufRead, stdin};

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .nth(0).unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    
    let (mut h, mut m) = (nums[0], nums[1]);

    m -= 45;

    if m < 0 {
        h -= 1;
        m += 60;
    }

    if h < 0 {
        h += 24;
    }

    println!("{} {}", h, m);
}
