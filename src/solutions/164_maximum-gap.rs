#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut max_gap = 0;

        for i in 1..nums.len() {
            max_gap = max(max_gap, nums[i] - nums[i - 1])
        }

        max_gap
    }
}
// end_submission
