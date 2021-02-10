use std::io;

fn main() {
    let reader = io::stdin();
    let c = read_cost(&reader);
    let l = read_header(&reader);

    let mut s = 0.;
    for _ in 0..l {
        let w = read_case(&reader);
        s += w.0 * w.1;
    }

    print!("{}", c * s);
}

fn read_header(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_cost(reader: &io::Stdin) -> f64 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_case(reader: &io::Stdin) -> (f64, f64) {
    let line = read_line(reader);
    let mut iter = 
        line.split_whitespace()
        .map(|x| x.parse().unwrap());

    (iter.next().unwrap(), iter.next().unwrap())
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}