use std::io::BufRead;

fn main() {
    let gnomes: Vec<i32> =
        std::io::stdin().lock().lines()
        .map(|x| {
            x.unwrap().trim_end().parse().unwrap()
        }).collect();

        for i in 0..8 {
            for j in i+1..9 {
                let mut buf = gnomes.clone();
                buf.remove(j);
                buf.remove(i);

                if buf.iter().sum::<i32>() == 100 {
                    for gnome in buf.into_iter() {
                        println!("{}", gnome);
                    }

                    return;
                }
            }
        }
}
