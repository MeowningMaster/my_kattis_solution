use std::io::{BufRead, stdin};

fn main() {
    let line = stdin()
    .lock()
    .lines()
    .nth(0).unwrap().unwrap();

    let mut cups = vec![1, 0, 0];

    for ch in line.chars() {
        match ch {
            'A' => cups.swap(0, 1),
            'B' => cups.swap(1, 2),
            'C' => cups.swap(0, 2),
            _ => panic!("irr input")
        }
    }

    let pos = cups.into_iter().position(|x| x == 1).unwrap();
    println!("{}", pos + 1);
}
