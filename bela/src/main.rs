use std::io;

fn main() {
    let cards = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7'];
    let scores = vec![11, 4, 3, 2, 10, 0, 0, 0];
    let dom_scores = vec![11, 4, 3, 20, 10, 14, 0, 0];

    let mut buff = String::new();
    let reader = io::stdin();

    reader.read_line(&mut buff).expect("err");
    let mut parts = buff.trim().split_whitespace();

    let n = parts.next().unwrap().parse::<i32>().unwrap();
    let dom_suit = parts.next().unwrap().chars().nth(0).unwrap();

    let mut total = 0;
    for _i in 0..n*4 {
        buff = String::new();
        reader.read_line(&mut buff).expect("err");
        let card = buff.chars().nth(0).unwrap();
        let suit = buff.chars().nth(1).unwrap();

        let number = cards.iter().position(|&x| x == card).unwrap();

        let score = if dom_suit == suit { dom_scores[number] } else { scores[number] };
        total += score;
    }

    println!("{}", total);
}


