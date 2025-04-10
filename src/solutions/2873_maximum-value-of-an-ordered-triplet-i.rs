#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_val = 0;

        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    max_val = max(max_val, (nums[i] - nums[j]) * nums[k]);
                }
            }
        }

        max_val
    }
}
// end_submission
