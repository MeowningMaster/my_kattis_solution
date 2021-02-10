use std::{fmt::format, io::{BufRead, stdin}};

fn main() {
    let reader = stdin();
    let line = reader.lock().lines().nth(0).unwrap().unwrap();

    let mut parts = line.split_whitespace();
    let y = parts.next().unwrap();
    let p = parts.next().unwrap();

    let ans = if &y[y.len()-2..] == "ex" {
        format!("{}{}", y, p)
    } else {
        match &y[y.len()-1..] {
            "e" => format!("{}x{}", y, p),
            "a" | "i" | "o" | "u" => format!("{}ex{}", &y[..y.len()-1], p),
            _ => format!("{}ex{}", y, p)
        }
    };

    println!("{}", ans);
}
