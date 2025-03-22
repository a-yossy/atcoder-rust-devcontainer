use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if a + b == c[i] {
            ans = i + 1;
            break;
        }
    };

    println!("{}", ans);
}
