use std::io::{BufRead, stdin};

fn main() {
    let line = stdin()
        .lock()
        .lines()
        .nth(0).unwrap().unwrap();
        
    let (mut w, mut u, mut l, mut c) = (0, 0, 0, 0);
    for ch in line.trim_end().chars() {
        match ch {
            '_' => w += 1,
            'A'..='Z' => u += 1,
            'a'..='z' => l += 1,
            _ => c += 1
        }
    }

    let (w, u, l, c) = (w as f64, u as f64, l as f64, c as f64);
    let len = line.len() as f64;
    println!("{}\n{}\n{}\n{}", w / len, l / len, u / len, c / len);
}