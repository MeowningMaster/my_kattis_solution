use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();

    let buf = read_nums(&reader);
    let (s, t, n) = (buf[0], buf[1], buf[2]);
    let d = read_nums(&reader);
    let b = read_nums(&reader);
    let c = read_nums(&reader);

    let mut timer = s + d[0];
    let d = &d[1..];

    for i in 0..n as usize {
        timer += (-timer).rem_euclid(c[i]) + b[i] + d[i];
    }

    let ans = if timer <= t {"yes"} else {"no"};
    println!("{}", ans);
}

fn read_nums(reader: &Stdin) -> Vec<i32> {
    read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
