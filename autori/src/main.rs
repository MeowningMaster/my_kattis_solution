use std::io;

fn main() {
    let input = io::stdin();
    let mut line = String::new();
    input.read_line(&mut line).expect("err");

    let names = line.split('-').collect::<Vec<&str>>();
    let mut short_form = String::new();
    for name in names {
        short_form.push(name.chars().nth(0).unwrap());
    }

    println!("{}", short_form);
}
