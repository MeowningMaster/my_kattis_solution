use std::{
    cmp::min,
    io::{stdin, Stdin},
};

fn main() {
    let reader = stdin();
    let line = read_line(&reader);

    let (mut t, mut c, mut g) = (0, 0, 0);
    for ch in line.chars() {
        match ch {
            'T' => t += 1,
            'G' => g += 1,
            'C' => c += 1,
            _ => (),
        }
    }

    let sets = min(t, min(g, c));
    let points = t * t + g * g + c * c + sets * 7;
    println!("{}", points);
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
