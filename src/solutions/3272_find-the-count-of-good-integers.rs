#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut h: HashSet<String> = HashSet::new();

        let c = 10i32.pow(((n - 1) / 2) as u32);
        let skip = (n & 1) as usize;

        for i in c..10 * c {
            let s = i.to_string();
            let rev: String = s.chars().rev().skip(skip).collect();

            let p = format!("{}{}", s, rev);

            if p.parse::<i64>().unwrap() % (k as i64) == 0 {
                let mut s_p: Vec<char> = p.chars().collect();
                s_p.sort();
                h.insert(s_p.into_iter().collect());
            }
        }

        let mut f = vec![1i64; (n + 1) as usize];
        for i in 1..=n as usize {
            f[i] = f[i - 1] * (i as i64);
        }

        let mut ans = 0i64;

        for s in h {
            let mut v = vec![0; 10];
            for c in s.chars() {
                v[c.to_digit(10).unwrap() as usize] += 1;
            }

            let mut res = (n as i64 - v[0] as i64) * f[(n - 1) as usize];
            for x in v {
                res /= f[x];
            }
            ans += res;
        }

        ans
    }
}
// end_submission
