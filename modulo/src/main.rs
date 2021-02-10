use std::{collections::HashSet, io::{stdin, BufRead}};

fn main() {
    let nums: Vec<i32> = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim_end().parse().unwrap())
        .collect();

    let s = nums
        .into_iter()
        .fold(HashSet::new(), |mut acc, x| {acc.insert(x % 42); acc});

    println!("{}", s.len());
}
