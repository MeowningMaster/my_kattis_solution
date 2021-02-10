use std::io::BufRead;

fn main() {
    let mut nums: Vec<i32> =
        std::io::stdin().lock().lines()
        .nth(0).unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    nums.sort();

    for i in nums[0]+1..=nums[1]+1 {
        println!("{}", i);
    }
}
