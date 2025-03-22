use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let ans = (n / 100_f64).ceil();

    println!("{}", ans);
}
