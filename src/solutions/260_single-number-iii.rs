#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let b = nums.iter().fold(0, |acc, x| acc ^ x);

        for i in 0..32 {
            if (b >> i) & 1 == 1 {
                return nums.iter().fold(vec![0, 0], |mut acc, x| {
                    if (x >> i) & 1 == 1 {
                        acc[0] ^= x;
                    } else {
                        acc[1] ^= x;
                    };
                    acc
                });
            }
        }

        unreachable!()
    }
}
// end_submission
