use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<String> =
        stdin().lock().lines()
            .map(|x| x.unwrap())
            .collect();

    let buf: Vec<i32> =
        lines[0].split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    let (n, t) = (buf[0], buf[1]);
    
    let a: Vec<i32> =
        lines[1].split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    
    println!("{}", solve(n, t, a));
}

fn solve(n: i32, t: i32, mut a: Vec<i32>) -> String {
    match t {
        1 => {
            a.retain(|&x| x <= 7777);
            a.sort();

            for x in a.iter() {
                let y = 7777 - x;
                if a.binary_search(&y).is_ok() {
                    return "Yes".to_string();
                }
            }

            return "No".to_string();
        },
        2 => {
            for i in 0..n as usize {
                for j in i+1..n as usize {
                    if a[i] == a[j] {
                        return "Contains duplicate".to_string();
                    }
                }
            }

            return "Unique".to_string();
        },
        3 => {
            let item = a[0];

            let mut cou = 0;
            for &i in a.iter() {
                if i == item {
                    cou += 1;
                }
            }

            if cou > n / 2 {
                return item.to_string();
            } else {
                let mut another_item = 0;
                for &i in a.iter() {
                    if i != item {
                        another_item = i;
                        break;
                    }
                }

                let mut cou = 0;
                for &i in a.iter() {
                    if i == another_item {
                        cou += 1;
                    }
                }

                if cou > n / 2 {
                    return another_item.to_string();
                } else {
                    return (-1).to_string();
                }
            }
        },
        4 => {
            a.sort();

            if n & 1 == 1 {
                return format!("{}", a[(n / 2) as usize]);
            } else {
                return format!("{} {}", a[(n / 2 - 1) as usize], a[(n / 2) as usize]);
            }
        },
        5 => {
            a.sort();

            let mut res = String::new();
            for i in a.into_iter() {
                if i >= 100 && i <= 999 {
                    res.push_str(i.to_string().as_str());
                    res.push(' ');
                }                
            }

            return res.trim_end().to_string();
        },
        _ => ()
    };

    "irr input".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        use super::solve;

        assert_eq!("Yes", solve(7, 1, vec![1, 7770, 3, 4, 5, 6, 7]));
        assert_eq!("Unique", solve(7, 2, vec![1, 2, 3, 4, 5, 6, 7]));
        assert_eq!("1", solve(7, 3, vec![1, 1, 1, 1, 2, 2, 2]));
        assert_eq!("4 5", solve(8, 4, vec![8, 1, 4, 3, 6, 7, 5, 2]));
        assert_eq!("210 321 543 777 999", solve(7, 5, vec![210, 999, 1000, 543, 321, 99, 777]));
    }
}