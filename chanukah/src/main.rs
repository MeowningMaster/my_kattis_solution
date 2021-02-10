use std::io;

fn main() {
    let reader = io::stdin();
    let p = read_head(&reader);

    for _ in 0..p {
        let (k, n) = read_case(&reader);
        println!("{} {}", k, ((1 + n) * n) / 2 + n);
    }
}

fn read_head(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_case(reader: &io::Stdin) -> (i32, i32) {
    let line = read_line(reader);
    let mut it =
        line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    (it.next().unwrap(), it.next().unwrap())
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
