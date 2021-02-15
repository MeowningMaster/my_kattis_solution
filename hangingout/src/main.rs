use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();
    let (l, x) = read_head(&reader);

    let mut count = 0;
    let mut denied = 0;
    for _ in 0..x {
        let (event, group_size) = read_case(&reader);
        match event.as_str() {
            "enter" => {
                let free = l - count;
                if group_size <= free {
                    count += group_size;
                } else {
                    denied += 1;
                }
            },
            "leave" => {
                count -= group_size;
            },
            _ => panic!()
        }
    }

    println!("{}", denied);
}

fn read_head(reader: &Stdin) -> (usize, usize) {
    let buf = read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    (buf[0], buf[1])
}

fn read_case(reader: &Stdin) -> (String, usize) {
    let line = read_line(reader);
    let mut parts = line.split_whitespace();
    let event = parts.next().unwrap().to_string();
    let group_size = parts.next().unwrap().parse().unwrap();

    (event, group_size)
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
