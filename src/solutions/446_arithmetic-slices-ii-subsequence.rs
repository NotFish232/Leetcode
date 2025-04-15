#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![HashMap::new(); nums.len()];
        let mut count = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;

                let j_val = *dp[j].get(&diff).unwrap_or(&0);

                *dp[i].entry(diff).or_insert(0) += j_val + 1;

                count += j_val;
            }
        }

        count
    }
}
// end_submission
