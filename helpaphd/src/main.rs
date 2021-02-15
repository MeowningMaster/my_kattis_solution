use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();
    let n = read_head(&reader);

    for _ in 0..n {
        let line = read_line(&reader);

        if line == "P=NP" {
            println!("skipped");
        } else {
            let sum = calc_sum(line);
            println!("{}", sum);
        }
    }
}

fn read_head(reader: &Stdin) -> usize {
    read_line(reader).parse().unwrap()
}

fn calc_sum(expr: String) -> i32 {
    let ns = expr
        .split('+')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    ns.into_iter().sum()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().to_string()
}
