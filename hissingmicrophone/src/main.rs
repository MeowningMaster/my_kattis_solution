use std::io;

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let mut chars = buf.trim_end().chars();

    let mut f = false;
    let mut cur = chars.next().unwrap();
    for ch in chars {
        if cur == 's' && ch == 's' {
            f = true;
            break;
        }

        cur = ch;
    }

    println!("{}", if f {"hiss"} else {"no hiss"});
}
