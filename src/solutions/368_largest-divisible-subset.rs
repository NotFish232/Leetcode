#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let mut dp = vec![1; nums.len()];
        let mut prev = vec![usize::MAX; nums.len()];
        let mut max_i = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }

            if dp[i] > dp[max_i] {
                max_i = i;
            }
        }

        let mut v = Vec::new();
        let mut p = max_i;

        while p != usize::MAX {
            v.push(nums[p]);
            p = prev[p];
        }

        v
    }
}
// end_submission
