#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let mut dp = vec![0; nums.len()];

        for (i, num) in nums.iter().enumerate() {
            if i < 3 {
                dp[i] = max(k - num, 0) as i64;
            } else {
                dp[i] = max(k - num, 0) as i64 + *dp[i - 3..i].iter().min().unwrap();
            }
        }

        *dp[dp.len() - 3..dp.len()].iter().min().unwrap()
    }
}
// end_submission
