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
        .collect::<Vec<u32>>();

    let (g, s, c) = (ns[0], ns[1], ns[2]);
    let power = g * 3 + s * 2 + c;

    let ans = match power {
        8..=std::u32::MAX => "Province or Gold",
        6 | 7 => "Duchy or Gold",
        5 => "Duchy or Silver",
        3 | 4 => "Estate or Silver",
        2 => "Estate or Copper",
        0 | 1 => "Copper"
    };

    println!("{}", ans);
}
