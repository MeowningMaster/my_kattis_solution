use std::io::{stdin, BufRead};

fn main() {
    let reader = stdin();
    let line = reader.lock().lines().nth(0).unwrap().unwrap();
    let n = line.trim_end().parse::<f64>().unwrap();

    let ans = n.powf(1. / n);
    println!("{}", ans);
}
