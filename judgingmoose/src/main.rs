use std::{cmp::max, io::{stdin, BufRead}};

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

    let (l, r) = (ns[0], ns[1]);

    match (l, r) {
        (0, 0) => println!("Not a moose"),
        (l, r) if l == r => println!("Even {}", l + r),
        (l, r) if l != r => println!("Odd {}", max(l, r) * 2),
        _ => ()
    }
}
