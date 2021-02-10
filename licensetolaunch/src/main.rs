use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();

    let _n = read_num(&reader);
    let nums = read_nums(&reader);

    let mut min = std::i32::MAX;
    let mut min_pos = 0;
    for (i, num) in nums.into_iter().enumerate().rev() {
        if min >= num {
            min = num;
            min_pos = i;
        }
    }

    println!("{}", min_pos);
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