use std::io;

fn main() {
    let reader = io::stdin();

    let t = read_int(&reader);

    for _ in 0..t {
        let n = read_int(&reader);

        let res = match n {
            1 => 1,
            2 => 2,
            3 => 6,
            4 => 4,
            _ => 0
        };

        println!("{}", res);
    }
}

fn read_int(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i32>().unwrap()
}

