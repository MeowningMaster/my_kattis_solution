use core::panic;
use std::io::{Stdin, stdin};

fn main() {
    let reader = stdin();
    let (r, c) = read_head(&reader);
    let map = read_map(&reader, r);

    let mut record = [0; 5];
    for i in 0..(r-1) {
        'case: for j in 0..(c-1) {
            let cells = [map[i][j], map[i+1][j], map[i][j+1], map[i+1][j+1]];
   
            let mut count = 0;
            for cell in cells.iter() {
                match cell {
                    CellType::Building => continue 'case,
                    CellType::Car => count += 1,
                    CellType::FreeSpace => ()
                }
            }

            record[count] += 1;
        }
    }

    for i in record.iter() {
        println!("{}", i);
    }
}

#[derive(Clone, Copy)]
enum CellType {
    FreeSpace,
    Car,
    Building
}

fn read_head(reader: &Stdin) -> (usize, usize) {
    let ns = read_line(reader)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    (ns[0], ns[1])
}

fn read_map(reader: &Stdin, height: usize) -> Vec<Vec<CellType>> {
    let mut map = Vec::new();

    for _ in 0..height {
        let mut row = Vec::new();
        let line = read_line(reader);

        for ch in line.trim_end().chars() {
            let cell = match ch {
                '.' => CellType::FreeSpace,
                'X' => CellType::Car,
                '#' => CellType::Building,
                _ => panic!()
            };

            row.push(cell);
        }

        map.push(row);
    }

    map
}

fn read_line(reader: &Stdin) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("err");

    buf
}
