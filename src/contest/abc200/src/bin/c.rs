use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut rest = Vec::new();
    for element in a {
        rest.push(element % 200);
    }

    let mut count: HashMap<usize, i128> = HashMap::new();

    let mut ans = 0;

    for element in rest {
        if let Some(now_count) = count.get(&element) {
            count.insert(element, now_count + 1);
        } else {
            count.insert(element, 1);
        }
    }

    for (_, value) in count {
        ans += value * (value - 1) / 2
    }

    println!("{}", ans);
}
