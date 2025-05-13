#[allow(unused)]
use crate::stubs::*;

struct Solution;

 // start_submission
impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut v = [0; 26];
        for &b in s.as_bytes() {
            v[(b - b'a') as usize] += 1;
        }

        for _ in 0..t {
            let mut new_v = [0; 26];

            for (i, c) in v.into_iter().enumerate() {
                if i == 25 {
                    new_v[0] = (new_v[0] + c) % Self::MOD;
                   new_v[1] = (new_v[1] + c) % Self::MOD;
                } else {
                   new_v[i + 1] = (new_v[i + 1] + c) % Self::MOD;
                }
            }

            v = new_v;
        }

       (v.into_iter().sum::<i64>() % Self::MOD) as i32
    }
}
 // end_submission
