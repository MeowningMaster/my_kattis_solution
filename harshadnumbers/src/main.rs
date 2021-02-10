use std::io;

fn main() {
    let reader = io::stdin();
    
    let n = read_int(&reader);

    for i in n..=1000000000 {
        if check_hashard(i) {
            println!("{}", i);
            break;
        }
    }
}

fn read_int(reader: &io::Stdin) -> i32 {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().parse::<i32>().unwrap()
}

fn check_hashard(i: i32) -> bool {
    let mut nums = Vec::new();

    let mut n = i.clone();
    while n > 0 {
        nums.push(n % 10);
        n /= 10;
    }
    
    i % nums.iter().sum::<i32>() == 0
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_hashard_test() {
        use super::check_hashard;

        assert!(check_hashard(24));
        assert!(check_hashard(987654330));

        assert!(!check_hashard(987654321));
    }
}