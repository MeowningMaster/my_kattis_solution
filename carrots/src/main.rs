use std::io;

fn main() {
    let mut buff = String::new();
    let reader = io::stdin();
    reader.read_line(&mut buff).expect("err");

    let mut parts = buff.trim().split_whitespace();
    parts.next();
    let carrots = parts.next().unwrap().parse::<i32>().unwrap();

    println!("{}", carrots);
}
