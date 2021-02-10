use std::{io::{BufRead, stdin}, mem::swap};

fn main() {
    let rings: Vec<i32> =
        stdin().lock().lines().nth(1).unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    for i in 1..rings.len() {
        let (a, b) = reduce(rings[0], rings[i]);
        println!("{}/{}", a, b);
    }
}

fn reduce(mut a: i32, mut b: i32) -> (i32, i32) {
    let gcd = gcd(a, b);
    a = a / gcd;
    b = b / gcd;

    (a, b)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        a %= b;
        swap(&mut a, &mut b);
    }

    a
}

#[cfg(test)]
mod tests {
    use crate::gcd;

    #[test]
    fn gcd_test() {
        assert_eq!(4, gcd(4, 12));
        assert_eq!(1, gcd(5, 13));
        assert_eq!(3, gcd(12, 33));
    }
}