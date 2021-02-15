use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim_end().to_string())
        .collect::<Vec<_>>();

    let teams = lines
        .into_iter()
        .skip(1)
        .map(|x| {
            let mut ps = x.split_whitespace();
            Team {
                univ: ps.next().unwrap().to_string(),
                name: ps.next().unwrap().to_string(),
            }
        })
        .collect::<Vec<_>>();

    let mut awarded_univs = HashSet::new();
    let mut awarded_count = 0;
    for team in teams {
        if awarded_univs.insert(team.univ.clone()) {
            println!("{} {}", team.univ, team.name);
            
            awarded_count += 1;
            if awarded_count == 12 {
                break;
            }
        }
    }
}

struct Team {
    univ: String,
    name: String,
}
