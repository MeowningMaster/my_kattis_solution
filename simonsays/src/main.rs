use std::io::{BufRead, stdin};

fn main() {
    let reader = stdin();
    let lines = reader
        .lock()
        .lines()
        .skip(1)
        .map(|x| x.unwrap());

    const SIMON: &str = "Simon says";
    const SIMON_LEN: usize = SIMON.len() - 1;

    for mut line in lines {
        if let Some(_) = line.find(SIMON) {
            line.replace_range(0..=SIMON_LEN, "");
            println!("{}", line);
        }
    }
}
