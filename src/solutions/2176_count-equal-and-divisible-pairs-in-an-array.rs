#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len())
            .map(|i| {
                (i + 1..nums.len())
                    .map(|j| (nums[i] == nums[j] && (i * j) as i32 % k == 0) as i32)
                    .sum::<i32>()
            })
            .sum()
    }
}
// end_submission
