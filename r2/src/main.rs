use std::io;

fn main() {
    let mut buff = String::new();
    let reader = io::stdin();
    reader.read_line(&mut buff).expect("err");

    let mut parts = buff.trim().split_whitespace();
    let r1 = parts.next().unwrap().parse::<i32>().unwrap();
    let r2 = parts.next().unwrap().parse::<i32>().unwrap();

    let r3 = r2*2 - r1;
    println!("{}", r3);
}