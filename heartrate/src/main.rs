use std::{io, mem};

fn main() {
    let reader = io::stdin();

    let n = read_int(&reader);

    for _ in 0..n {
        let (b, p) = read_case(&reader);

        let b = b as f64;
        let bpm = 60. * b / p;
        let mut min_abpm = 60. / (p / (b - 1.));
        let mut max_abpm = 60. / (p / (b + 1.));

        if min_abpm > max_abpm {
            mem::swap(&mut min_abpm, &mut max_abpm);
        }

        println!("{} {} {}", min_abpm, bpm, max_abpm);
    }
}

fn read_int(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i32>().unwrap()
}

fn read_case(reader: &io::Stdin) -> (i32, f64) {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let mut parts = buf.trim_end().split_whitespace();
    let b = parts.next().unwrap().parse::<i32>().unwrap();
    let p = parts.next().unwrap().parse::<f64>().unwrap();

    (b, p)
}