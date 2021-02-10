use std::{cmp::max, io};

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let mut nums = 
        buf
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
    
    let n = nums.next().unwrap();
    let h = nums.next().unwrap();
    let v = nums.next().unwrap();

    let max_h = max(h, n - h);
    let max_v = max(v, n - v);

    let max_sq = max_h * max_v;
    println!("{}", max_sq * 4);
}
