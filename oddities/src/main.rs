use std::io::{Stdin, stdin, Error};

fn main() {
    let reader = stdin();
    let n = read_num(&reader).expect("irr header");

    for _ in 0..n {
        let x = read_num(&reader).expect("irr data");
        
        let res = if x % 2 == 0 {
            "even"
        } else {
            "odd"
        };

        println!("{} is {}", x, res);
    }
}

fn read_num(reader: &Stdin) -> Result<i32, Error> {
    Ok(read_line(reader)?.trim_end().parse().unwrap())
}

fn read_line(reader: &Stdin) -> Result<String, Error> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    Ok(buf)
}
