use std::{collections::HashSet, io::{Stdin, stdin}};

fn main() {
    let reader = stdin();

    let t = read_num(&reader);

    for _ in 0..t {
        let n = read_num(&reader);
        let mut cities = HashSet::new();

        for _ in 0..n {
            let city = read_line(&reader);
            cities.insert(city);
        }

        println!("{}", cities.len());
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