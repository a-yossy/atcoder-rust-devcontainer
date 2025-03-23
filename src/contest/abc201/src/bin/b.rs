use proconio::input;

fn main() {
    input! {
        n: usize,
        mut data: [(String, i32); n]
    }

    data.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", data[1].0);
}
