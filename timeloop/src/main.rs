fn main() {
    let n = read_number();

    for i in 1..=n {
        println!("{} Abracadabra", i);
    }
}

fn read_number() -> i32 {
    use std::io;
    
    let input = io::stdin();
    let mut line = String::new();
    input.read_line(&mut line).expect("err");

    line.trim().parse::<i32>().unwrap()
}