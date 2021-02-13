use std::{collections::HashSet, io::{Stdin, stdin}};

fn main() {
    let reader = stdin();
    let n = read_num(&reader);

    for i in 1..=n {
        let _g = read_num(&reader);
        let guests = read_nums(&reader);

        let mut count = HashSet::new();
        for guest in guests.into_iter() {
            if let Some(_) = count.get(&guest) {
                count.remove(&guest);
            } else {
                count.insert(guest);
            }
        }
        
        let unpaired = count.into_iter().next().unwrap();
        println!("Case #{}: {}", i, unpaired);
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_nums(reader: &Stdin) -> Vec<i32> {
    read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
