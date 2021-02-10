use std::io::{BufRead, stdin};

fn main() {
    let reader = stdin();
    let line = reader.lock().lines().nth(0).unwrap().unwrap();

    let mut occurs = vec![0; 13];

    let cards = line.split_whitespace().collect::<Vec<&str>>();
    for card in cards.into_iter() {
        occurs[get_pos(card.chars().nth(0).unwrap())] += 1;
    }

    let max = occurs.into_iter().max().unwrap();
    println!("{}", max);
}

fn get_pos(rank: char) -> usize {
    match rank {
        'A' => 0,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        _ => rank.to_digit(10).unwrap() as usize - 1
    }
}