#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap();

        let mut c = 0;
        let mut max_c = 0;

        for &x in &nums {
            if x == max_val {
                c += 1;
            } else {
                c = 0;
            }

            max_c = max(max_c, c);
        }

        max_c
    }
}
// end_submission
