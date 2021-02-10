use std::io;

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let mut nums =
        buf
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

    let x = nums.next().unwrap();
    let y = nums.next().unwrap();
    let n = nums.next().unwrap();

    for i in 1..=n {
        match i {
            i if i % x == 0 && i % y == 0 => println!("FizzBuzz"),
            i if i % x == 0 => println!("Fizz"),
            i if i % y == 0 => println!("Buzz"),
            i => println!("{}", i)
        }
    }
}
