use std::io::{stdin, BufRead};

fn main() {
    let line = stdin().lock().lines().next().unwrap().unwrap();
    let mid = line.len() / 2;
    let (a, b) = (rotate(&line[..mid]), rotate(&line[mid..]));
    let c = merge(a, b);

    println!("{}", c);
}

trait Alphabetical {
    fn index(&self) -> u8;
    fn from_index(index: u8) -> Self;
}

impl Alphabetical for char {
    fn index(&self) -> u8 {
        *self as u8 - 'A' as u8
    }

    fn from_index(index: u8) -> Self {
        ('A' as u8 + index) as char
    }
}

fn rotate(seq: &str) -> String {
    let mut rotation = 0 as u8;
    for ch in seq.chars() {
        rotation += ch.index();
        rotation %= 26;
    }

    let mut res = String::new();
    for ch in seq.chars() {
        let rot_ch = char::from_index((ch.index() + rotation) % 26);
        res.push(rot_ch);
    }

    res
}

fn merge(a: String, b: String) -> String {
    let mut a_chs = a.chars();
    let mut b_chs = b.chars();

    let mut res = String::new();
    while let (Some(a_ch), Some(b_ch)) = (a_chs.next(), b_chs.next()) {
        let rot_ch = char::from_index((a_ch.index() + b_ch.index()) % 26);
        res.push(rot_ch);
    }

    res
}