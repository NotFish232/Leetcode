#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut count = 0;

        for n in low..=high {
            let s = n.to_string();

            if s.len() % 2 == 0 {
                let s1 = s.as_bytes().iter().take(s.len() / 2).sum::<u8>();
                let s2 = s.as_bytes().iter().skip(s.len() / 2).sum::<u8>();

                count += (s1 == s2) as i32;
            }
        }

        count
    }
}
// end_submission
