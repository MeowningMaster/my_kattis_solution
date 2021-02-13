use std::io::{BufRead, stdin};

fn main() {
    let x = stdin()
        .lock()
        .lines()
        .nth(0).unwrap().unwrap()
        .trim_end().parse::<f64>().unwrap();

    let rom = x * 1000. * 5280. / 4854.;
    println!("{}", rom.round());
}