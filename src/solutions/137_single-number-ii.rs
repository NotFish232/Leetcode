#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut v = [0; 32];

        for n in nums {
            for i in 0..32 {
                if (n >> i) & 1 == 1 {
                    v[i] = (v[i] + 1) % 3;
                }
            }
        }

        v.into_iter()
            .enumerate()
            .fold(0, |acc, (i, n)| if n == 1 { acc + (1 << i) } else { acc })
    }
}
// end_submission
