use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();

    let w = read_num(&reader);
    let n = read_num(&reader);

    let mut sq = 0;
    for _ in 0..n {
        let (wi, li) = read_nums(&reader);
        sq += wi * li;
    }

    let l = sq / w;
    println!("{}", l);
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_nums(reader: &Stdin) -> (i32, i32) {
    let ns = read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    (ns[0], ns[1])
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
