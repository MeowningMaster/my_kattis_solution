use std::io::{stdin, BufRead};

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim_end().parse().unwrap())
        .collect::<Vec<i32>>();
    let (l, d, x) = (nums[0], nums[1], nums[2]);

    for i in l..=d {
        if num_sum(i) == x {
            println!("{}", i);
            break;
        }
    }

    for i in (l..=d).rev() {
        if num_sum(i) == x {
            println!("{}", i);
            break;
        }
    }
}

fn num_sum(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 0 {
        res += n % 10;
        n /= 10;
    }

    res
}
