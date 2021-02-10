use std::io;

fn main() {
    let reader = io::stdin();
    let n = read_head(&reader);

    for _ in 0..n {
        let (_, o) = read_case(&reader);
        
        let mut c = 1;
        for i in o.into_iter() {
            c += i - 1;
        }

        println!("{}", c);
    }
}

fn read_head(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_case(reader: &io::Stdin) -> (i32, Vec<i32>) {
    let line = read_line(reader);
    let mut it =
        line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let k = it.next().unwrap();
    let o = it.collect();

    (k, o)
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
