fn main() {
    let winner = 
        if read_number() % 2 == 0 { "Bob" } else { "Alice" };
    println!("{}", winner);
}

fn read_number() -> i32 {
    use std::io;
    
    let input = io::stdin();
    let mut line = String::new();
    input.read_line(&mut line).expect("err");

    line.trim().parse::<i32>().unwrap()
}