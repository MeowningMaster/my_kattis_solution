use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();
    
    let n = read_num(&reader);

    for _ in 0..n {
        let l = read_line(&reader);
        println!("{}", l.len());
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).parse().unwrap()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().to_string()
}