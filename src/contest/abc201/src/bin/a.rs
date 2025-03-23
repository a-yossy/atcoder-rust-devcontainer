use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        mut a: [usize; 3],
    }

    a.sort_by(|a, b| b.cmp(a));
    let mut set = HashSet::new();
    for i in 0..2 {
        set.insert(a[i] - a[i + 1]);
    }

    if set.len() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
