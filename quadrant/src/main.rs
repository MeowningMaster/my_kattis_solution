use std::io;

fn main() {
    let reader = io::stdin();
    let x = read_number(&reader);
    let y = read_number(&reader);

    if x == 0 || y == 0 {
        panic!("irr input");
    }

    let quadrant = 
    if x > 0 {if y > 0 {1} else {4}}
    else {if y > 0 {2} else {3}};

    println!("{}", quadrant);
}

fn read_number(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i32>().unwrap()
}