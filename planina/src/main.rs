fn main() {
    let n = read_int();
    let dots = pow2(n) + 1;
    let res = dots * dots;

    println!("{}", res);
}

fn read_int() -> i64 {
    use std::io;

    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i64>().unwrap()
}

fn pow2(exp: i64) -> i64 {
    let mut res = 1;

    for _ in 0..exp {
        res <<= 1;
    }

    res
}