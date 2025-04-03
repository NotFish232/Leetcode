#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut num_pos = 0;
        let mut num_neg = 0;

        for num in nums {
            if num > 0 {
                num_pos += 1;
            } else if num < 0 {
                num_neg += 1;
            }
        }

        max(num_pos, num_neg)
    }
}
// end_submission
