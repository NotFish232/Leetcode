#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp1 = vec![0; nums.len()];
        dp1[1] = nums[0];

        for i in 1..nums.len() - 1 {
            dp1[i + 1] = max(dp1[i - 1] + nums[i], dp1[i]);
        }

        let mut dp2 = vec![0; nums.len()];
        dp2[nums.len() - 2] = nums[nums.len() - 1];

        for i in (1..nums.len() - 1).rev() {
            dp2[i - 1] = max(dp2[i + 1] + nums[i], dp2[i]);
        }

        max(dp1[dp1.len() - 1], dp2[0])
    }
}
// end_submission
