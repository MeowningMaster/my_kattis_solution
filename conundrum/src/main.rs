use std::io::{stdin, BufRead};

fn main() {
    let reader = stdin();
    let line = reader.lock().lines().nth(0).unwrap().unwrap().trim_end().to_string();

    let per = "PER".to_string();
    let mut per_pos = 0;
    let mut days = 0;
    for ch in line.chars() {
        if ch != per.chars().nth(per_pos).unwrap() {
            days += 1;
        }

        per_pos += 1;
        if per_pos == per.len() {
            per_pos = 0;
        }
    }

    println!("{}", days);
}
