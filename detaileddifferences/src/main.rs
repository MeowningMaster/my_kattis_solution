use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();
    let n = read_num(&reader);

    for _ in 0..n {
        let sample = read_line(&reader);
        let test = read_line(&reader);

        let len = sample.len();
        let mut sample_chars = sample.chars();
        let mut test_chars = test.chars();
        let mut ans = String::new();

        for _ in 0..len {
            let s_ch = sample_chars.next().unwrap();
            let t_ch = test_chars.next().unwrap();
            
            let ch = if s_ch == t_ch {
                '.'
            } else {
                '*'
            };

            ans.push(ch);
        }

        println!("{}", sample);
        println!("{}", test);
        println!("{}", ans);
        println!();
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).parse().unwrap()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf.trim_end().to_string()
}
