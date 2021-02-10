use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf).expect("err");
    let mut nums: Vec<i32> =
        buf
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    let c = &mut nums[1..];
    c.sort();

    let mut sum: i32 = c.iter().sum();

    let mut r = c.len() - 3;
    loop {
        sum -= c[r];

        if r >= 3 {
            r -= 3;
        } else {
            break;
        }
    }

    println!("{}", sum);
}
