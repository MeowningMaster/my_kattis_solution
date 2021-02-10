use std::io;

fn main() {
    let reader = io::stdin();
    let (n, w, h) = read_head(&reader);
    let m = (((w * w + h * h) as f32).sqrt()) as i32;

    for _ in 0..n {
        let n = read_case(&reader);

        let ans = if n <= m {
            "DA"
        } else {
            "NE"
        };

        println!("{}", ans);
    }
}

fn read_head(reader: &io::Stdin) -> (i32, i32, i32) {
    let nums: Vec<i32> = read_line(reader)
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();

    (nums[0], nums[1], nums[2])
}

fn read_case(reader: &io::Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

#[inline]
fn read_line(reader: &io::Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}