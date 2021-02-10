use std::{io, cmp};

fn main() {
    let reader = io::stdin();

    let n = read_int(&reader);

    for _ in 0..n {
        let (r, e, c) = read_case(&reader);

        let message = match c.cmp(&(e - r)) {
            cmp::Ordering::Equal => "does not matter",
            cmp::Ordering::Greater => "do not advertise",
            cmp::Ordering::Less => "advertise"
        };

        println!("{}", message);
    }
}

fn read_int(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i32>().unwrap()
}

fn read_case(reader: &io::Stdin) -> (i32, i32, i32) {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");
    
    let nums =
        buf
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (nums[0], nums[1], nums[2])
}