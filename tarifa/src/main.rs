fn main() {
    let x = read_number();
    let n = read_number();

    let mut used = 0;
    for _ in 0..n {
        used += read_number();
    }

    let unused = x * (n + 1) - used;

    println!("{}", unused);
}

fn read_number() -> i32 {
    use std::io;
    
    let input = io::stdin();
    let mut line = String::new();
    input.read_line(&mut line).expect("err");

    line.trim().parse::<i32>().unwrap()
}