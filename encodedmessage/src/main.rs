use std::io::{stdin, Stdin};

fn main() {
    let reader = stdin();
    let t = read_num(&reader);

    for _ in 0..t {
        let res = solve_case(read_line(&reader));
        println!("{}", res);
    }
}

fn solve_case(line: String) -> String {
    let len = line.len();
    let side = len.sqrt();

    let mut sqr = Vec::new();

    let mut chars = line.chars();
    for _ in 0..side {
        let mut row = Vec::new();
        for _ in 0..side {
            row.push(chars.next().unwrap());
        }

        sqr.push(row);
    }

    let mut res = String::new();
    for i in (0..side).rev() {
        for j in 0..side {
            res.push(sqr[j][i]);
        }
    }

    res
}

trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for usize {
    fn sqrt(&self) -> usize {
        ((*self as f32).sqrt()) as usize
    }
}

fn read_num(reader: &Stdin) -> i32 {
    read_line(reader).trim_end().parse().unwrap()
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt() {
        assert_eq!(2, 4.sqrt());
        assert_eq!(4, 16.sqrt());
        assert_eq!(3, 15.sqrt());
    }

    #[test]
    fn solve() {
        assert_eq!("TOPSECRET", solve_case("RSTEEOTCP".into()));
        assert_eq!(
            "RosesAreRedVioletsAreBlue",
            solve_case("eedARBtVrolsiesuAoReerles".into())
        );
    }
}
