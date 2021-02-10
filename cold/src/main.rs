use std::io;

fn main() {
    let reader = io::stdin();
    let _n = read_header(&reader);
    let a = read_cases(&reader);

    let mut c = 0;
    for i in a.into_iter() {
        if i < 0 {
            c += 1;
        }
    }

    println!("{}", c);
}

fn read_header(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_cases(reader: &io::Stdin) -> Vec<i32> {
    read_line(reader).split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}