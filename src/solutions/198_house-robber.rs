#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut p = vec![0; nums.len() + 1];
        p[1] = nums[0];

        for i in 1..nums.len() {
            p[i + 1] = max(p[i - 1] + nums[i], p[i]);
        }

        p[nums.len()]
    }
}
// end_submission
