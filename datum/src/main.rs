use std::io::{stdin, BufRead};

fn main() {
    let ns = stdin()
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    const DAYS_IN_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_OF_WEEK: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]; 
    const FIRST_DAY_OF_YEAR: i32 = 3;
    let (d, m) = (ns[0], ns[1]);

    let mut days = d;
    for i in 0..(m-1) as usize {
        days += DAYS_IN_MONTH[i];
    }

    let day = ((days + FIRST_DAY_OF_YEAR - 1) % 7) as usize;
    println!("{}", DAYS_OF_WEEK[day]);
}
