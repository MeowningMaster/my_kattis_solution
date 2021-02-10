use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();
    let t = read_num(&reader);

    for _ in 0..t {
        let nums = read_nums(&reader);

        let mut incomers = 0;
        let mut prev = 1;
        for num in nums.into_iter() {
            match num {
                0 | 1 => continue,
                _ => {
                    let bound = prev * 2;
                    
                    if num > bound {
                        incomers += num - bound;
                    }
                }
            }

            prev = num;
        }

        println!("{}", incomers);
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_nums(reader: &Stdin) -> Vec<i32> {
    read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
