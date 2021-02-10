use std::io;

fn main() {
    let reader = io::stdin();
    let n = read_number(&reader);

    let mut total = 0.;
    for _ in 0..n {
        let (quality, period) = read_period(&reader);
        total += quality * period;
    }

    println!("{}", total);
}

fn read_number(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");
    buf.trim_end().parse::<i32>().unwrap()
}

fn read_period(reader: &io::Stdin) -> (f64, f64) {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");
    let nums = 
        buf
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

    (nums[0], nums[1])
}