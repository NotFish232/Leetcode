#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn sums_to(nums: &[i32], n: i32) -> Vec<HashSet<usize>> {
        let mut dp = vec![vec![usize::MAX; n as usize + 1]; nums.len()];

        for (i, &x) in nums.iter().enumerate() {
            if x <= n {
                dp[i][x as usize] = i;
            }
        }

        for i in 1..nums.len() {
            for j in 0..i {
                for l in 0..=n as usize {
                    if dp[j][l] != usize::MAX && l as i32 + nums[i] <= n {
                        dp[i][l + nums[i] as usize] = j;
                    }
                }
            }
        }

        let mut subsets = Vec::new();

        for (i, v) in dp.iter().enumerate() {
            if v[n as usize] != usize::MAX {
                let (mut p, mut val) = (i, n);

                let mut subset = HashSet::new();

                loop {
                    subset.insert(p);

                    if nums[p] == val {
                        break;
                    }

                    val -= nums[p];
                    p = dp[p][(val + nums[p]) as usize];
                }

                subsets.push(subset);
            }
        }

        subsets
    }
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();

        if sum % 2 == 1 {
            return false;
        }

        let subsets = Self::sums_to(&nums, sum / 2);

        !subsets.is_empty()
    }
}
// end_submission
