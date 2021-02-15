use std::io::{stdin, BufRead};

fn main() {
    let ns = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let (_x, y) = (ns[0], ns[1]);
    let ans = if y & 1 == 0 {"possible"} else {"impossible"};
    println!("{}", ans);
}
