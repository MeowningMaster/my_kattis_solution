use std::{collections::HashSet, io::{BufRead, stdin}};

fn main() {
    let line = stdin().lock().lines().nth(0).unwrap().unwrap();
    let mut set = HashSet::<String>::new();

    let words = line.split_whitespace();
    for word in words.into_iter() {
        if let Some(_) = set.replace(word.to_string()) {
            println!("no");
            return;
        }
    }

    println!("yes");
}
