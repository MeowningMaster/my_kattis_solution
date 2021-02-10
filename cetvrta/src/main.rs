use std::io::BufRead;

fn main() {
    let nums: Vec<Vec<i32>> =
        std::io::stdin().lock().lines()
        .map(|x| {
            x.unwrap().split_whitespace()
            .map(|y| y.parse().unwrap())
            .collect()
        }).collect();

    let x = if nums[0][0] == nums[1][0] {
        nums[2][0]
    } else if nums[0][0] == nums[2][0] {
        nums[1][0]
    } else {
        nums[0][0]
    };

    let y = if nums[0][1] == nums[1][1] {
        nums[2][1]
    } else if nums[0][1] == nums[2][1] {
        nums[1][1]
    } else {
        nums[0][1]
    };

    println!("{} {}", x, y);
}
