use std::{
    collections::HashMap,
    io::{stdin, Stdin},
};

#[derive(PartialEq, Eq, Hash)]
struct BDay {
    day: u8,
    month: u8,
}

fn main() {
    let mut bdays = HashMap::<BDay, (String, i32)>::new();

    let reader = stdin();
    let n = read_num(&reader);

    for _ in 0..n {
        let (name, like, bday) = read_case(&reader);
        match bdays.get_mut(&bday) {
            Some(x) => {
                if x.1 < like {
                    x.0 = name;
                    x.1 = like;
                }
            }
            None => {
                bdays.insert(bday, (name, like));
            }
        }
    }

    let mut names = bdays.into_iter().map(|x| x.1 .0).collect::<Vec<String>>();
    names.sort();

    println!("{}", names.len());
    for name in names.into_iter() {
        println!("{}", name);
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_case(reader: &Stdin) -> (String, i32, BDay) {
    let line = read_line(reader);
    let mut parts = line.split_whitespace();

    let name = parts.next().unwrap().to_string();
    let like = parts.next().unwrap().parse::<i32>().unwrap();

    let mut bday_parts = parts.next().unwrap().split('/');
    let bday = BDay {
        day: bday_parts.next().unwrap().parse().unwrap(),
        month: bday_parts.next().unwrap().parse().unwrap(),
    };

    (name, like, bday)
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
