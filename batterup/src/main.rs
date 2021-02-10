use std::io;

fn main() {
    let mut buff = String::new();
    let reader = io::stdin();

    reader.read_line(&mut buff).expect("err");
    let mut n = buff.trim().parse::<i32>().unwrap();

    buff = String::new();
    reader.read_line(&mut buff).expect("err");
    let actions = buff.trim().split_whitespace();

    let mut sum = 0;
    for action in actions {
        let number = action.parse::<i32>().unwrap();

        if number != -1 {
            sum += number;
        } else {
            n -= 1;
        }
    }

    let res = sum as f64 / n as f64;
    println!("{}", res);
}