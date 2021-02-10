// unsolved yet

fn main() {
    let lines = get_lines();
    let nums = get_collection(&lines);

    let res = solve(nums);
    println!("{}", res);
}

fn solve(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let a = get_counted_collection(nums);

    let n = a.len();
    let mut res = 0;
    for i in 0..n as usize {
        for j in i..n as usize {
            let x = a[i].0 + a[j].0;
            let r = a.binary_search_by_key(&x, |&(a, _)| a);
            
            if let Ok(k) = r {
                // crop here
                res += a[i].1 * a[i].1 * a[k].1;
            }
        }
    }

    res * 2
}

fn get_lines() -> Vec<String>{
    use std::io::{self, BufRead};

    io::stdin().lock().lines()
        .map(|x| x.unwrap().trim_end().to_string())
        .collect()
}

fn get_collection(lines: &Vec<String>) -> Vec<i32> {
    lines[1].split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_counted_collection(sorted_collection: Vec<i32>) -> Vec<(i32, i32)> {
    let mut buf = Vec::<(i32, i32)>::new();

    if sorted_collection.len() > 0 {
        let mut current = sorted_collection[0];
        let mut counter = 1;

        for i in 1..sorted_collection.len() {
            if current == sorted_collection[i] {
                counter += 1;
            } else {
                buf.push((current, counter));
                current = sorted_collection[i];
                counter = 1; 
            }
        }
        buf.push((current, counter));
    }

    buf
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn solve_test() {
        assert_eq!(4, solve(vec![1, 2, 3, 4]));
        assert_eq!(10, solve(vec![1, 1, 3, 3, 4, 6]));
        assert_eq!(2, solve(vec![1, 0, -1]));
        assert_eq!(6, solve(vec![0, 0, 0]));
        assert_eq!(16, solve(vec![1, 1, 5, 5, 6, 6]));
        assert_eq!(176, solve(vec![-3, -3, -3, -2, -2, -1, 0, 1, 1, 1, 2, 2, 3]));
    }
}