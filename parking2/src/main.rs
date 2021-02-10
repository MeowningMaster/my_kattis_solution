use std::io;

fn main() {
    let reader = io::stdin();
    let t = read_num(&reader);

    for _ in 0..t {
        let _n = read_num(&reader);
        let a = read_vec(&reader);

        let (mut min, mut max) = (a[0], a[0]);
        for i in a.into_iter() {
            if min > i {
                min = i;
            } else if max < i {
                max = i;
            }
        }

        println!("{}", (max - min) * 2);
    }
}

fn read_num(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_vec(reader: &io::Stdin) -> Vec<i32> {
    read_line(reader).split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}