use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let half = n / 2;

    let mut win_t = 0;
    let mut win_a = 0;

    for i in 0..n {
        if s.chars().nth(i).unwrap() == 'T' {
            win_t += 1;
        } else {
            win_a += 1;
        }

        if n % 2 == 0 {
            if win_t == half {
                println!("T");

                return;
            }
            if win_a == half {
                println!("A");

                return;
            }
        }
    }

    if win_a > win_t {
        println!("A");
    } else {
        println!("T");
    }
}
