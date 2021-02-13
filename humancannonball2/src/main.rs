use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();
    let n = read_head(&reader);

    for _ in 0..n {
        let (v0, theta, x1, h1, h2) = read_case(&reader);
        let theta = theta.to_radians();

        const G: f64 = 9.81;
        let t = x1 / (v0 * theta.cos());
        let y = v0 * t * theta.sin() - 1. / 2. * G * t * t;

        let ans = if y - h1 >= 1. && h2 - y >= 1. {
            "Safe"
        } else {
            "Not safe"
        };

        println!("{}", ans);
    }
}

fn read_head(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_case(reader: &Stdin) -> (f64, f64, f64, f64, f64) {
    let nums = read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<f64>>();

    (nums[0], nums[1], nums[2], nums[3], nums[4])
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
