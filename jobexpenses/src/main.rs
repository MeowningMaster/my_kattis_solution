use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();

    let _n = read_num(&reader);
    let nums = read_nums(&reader);

    let mut exps = 0;
    for num in nums.into_iter() {
        if num < 0 {
            exps -= num;
        }
    }

    println!("{}", exps);
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_nums(reader: &Stdin) -> Vec<i32> {
    read_line(reader).split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}