use std::io;

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let mut nums =
        buf
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
    nums.sort();
    
    buf = String::new();
    reader.read_line(&mut buf).expect("err");
    
    for ch in buf.trim_end().chars() {
        let res = match ch {
            'A' => nums[0],
            'B' => nums[1],
            'C' => nums[2],
            _ => panic!("irr char")
        };

        println!("{} ", res);
    }
}
