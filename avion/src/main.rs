use std::io::{stdin, BufRead};

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    
    let mut f = false;
    for (i, line) in lines.iter().enumerate() {
        if let Some(_) = line.find("FBI") {
            f = true;
            print!("{} ", i + 1);
        }
    }

    if !f {
        print!("HE GOT AWAY!");
    }
}
