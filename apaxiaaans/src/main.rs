use std::io::{BufRead, stdin};

fn main() {
    let line = stdin()
        .lock()
        .lines()
        .nth(0).unwrap().unwrap();
    let mut chars = line.trim_end().chars();

    let mut prev = chars.next().unwrap();
    let mut res = String::new();
    res.push(prev);
    for ch in chars {
        if ch != prev {
            res.push(ch);
            prev = ch;
        }
    }

    println!("{}", res);
}
