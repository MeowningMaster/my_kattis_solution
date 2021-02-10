use std::io;

use io::Read;

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.lock().read_to_string(&mut buf).expect("err");

    let mut nums = 
        buf
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
    
    let mut max_i = 0;
    let mut max_score = 0;
    for i in 1..=5 {
        let mut score = 0;

        for _ in 1..=4 {
            score += nums.next().unwrap();
        }

        if max_score < score {
            max_i = i;
            max_score = score;
        }
    }

    println!("{} {}", max_i, max_score);
}
