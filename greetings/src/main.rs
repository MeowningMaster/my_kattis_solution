use std::io;

fn main() {
    let reader = io::stdin();
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    let e_num = buf.trim_end().len() - 2;

    let mut res = String::new();
    res.push('h');
    for _ in 0..e_num*2 {
        res.push('e');
    }
    res.push('y');

    println!("{}", res);
}
