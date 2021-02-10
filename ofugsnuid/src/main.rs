use std::io::{stdin, BufRead};

fn main() {
    let a: Vec<i32> = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim_end().parse::<i32>().unwrap())
        .skip(1)
        .collect();

    for i in a.into_iter().rev() {
        println!("{}", i);
    }
}
