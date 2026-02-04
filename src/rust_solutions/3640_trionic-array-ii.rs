#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();

        let mut dp = vec![vec![i64::MIN / 2; nums.len()]; 3];

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                dp[0][i] = max(dp[0][i - 1] + nums[i], nums[i - 1] + nums[i]);
                dp[2][i] = max(dp[2][i - 1] + nums[i], dp[1][i - 1] + nums[i]);
            }
            if nums[i] < nums[i - 1] {
                dp[1][i] = max(dp[1][i - 1] + nums[i], dp[0][i - 1] + nums[i]);
            }
        }

        *dp[2].iter().max().unwrap()
    }
}
// end_submission
