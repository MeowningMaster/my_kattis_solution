use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();

    let n = read_num(&reader);

    for _ in 0..n {
        let (a, b, c) = read_nums(&reader);

        if a + b == c || a + c == b || b + c == a || a * b == c || a * c == b || b * c == a {
            println!("Possible");
        } else {
            println!("Impossible");
        }
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_nums(reader: &Stdin) -> (i32, i32, i32) {
    let nums = read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    (nums[0], nums[1], nums[2])
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
