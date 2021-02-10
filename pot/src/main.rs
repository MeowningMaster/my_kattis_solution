use std::io::{stdin, Read};

fn main() {
    let reader = stdin();
    let mut buf = String::new();
    reader.lock().read_to_string(&mut buf).expect("err");
    let mut numbers = 
        buf
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap());
    
    let n = numbers.next().unwrap();

    let mut total: i64 = 0;
    for _ in 0..n {
        let p = numbers.next().unwrap();
        let base = p / 10;
        let exp = p % 10;

        let res = pow(base, exp);
        total += res;
    }

    println!("{}", total);
}

// iterative binpow
fn pow(mut base: i64, mut exp: i64) -> i64 {
    let mut res: i64 = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            res *= base;
        }

        base *= base;
        exp >>= 1;
    }

    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn pow_test() {
        use super::pow;

        assert_eq!(1, pow(1, 0));
        assert_eq!(1, pow(1, 1));

        assert_eq!(5, pow(5, 1));
        assert_eq!(1000, pow(10, 3));
        assert_eq!(3486784401, pow(9, 10));
    }
}