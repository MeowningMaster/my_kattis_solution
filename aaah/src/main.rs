use std::io::{stdin, BufRead};

fn main() {
    let ahs = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim_end().to_string())
        .collect::<Vec<_>>();

    let (a, b) = (ahs[0].len(), ahs[1].len());
    let ans = if a >= b { "go" } else { "no" };
    println!("{}", ans);
}
