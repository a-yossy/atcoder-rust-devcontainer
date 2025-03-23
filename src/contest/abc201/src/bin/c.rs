use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut maru_set = HashSet::new();
    for (i, char) in s.chars().enumerate() {
        if char == 'o' {
            maru_set.insert(i);
        }
    }

    let mut ans = 0;
    for password in 0..=9999 {
        let password = format!("{password:0>4}");
        let mut password_set = HashSet::new();
        let mut flag = true;
        for password_i in password.chars() {
            if let Some(index) = password_i.to_digit(10) {
                password_set.insert(index as usize);
                if vec!['o', '?'].contains(&s.chars().nth(index as usize).unwrap()) {
                    continue;
                } else {
                    flag = false;
                }
            }
        }

        for i in &maru_set {
            if !password_set.contains(i) {
                flag = false;
            }
        }

        if flag {
            ans += 1;
        }
    }

    println!("{}", ans);
}
