use std::io::{stdin, BufRead};

fn main() {
    let ns = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let p = ns.iter().sum::<u32>() as f64 / 2.;
    let ss = ns.iter().map(|&x| p - x as f64).product::<f64>();
    let s = ss.sqrt();

    println!("{}", s);
}
