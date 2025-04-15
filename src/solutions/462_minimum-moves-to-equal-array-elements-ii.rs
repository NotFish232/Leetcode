#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        (0..nums.len())
            .map(|i| (nums[i] - nums[nums.len() / 2]).abs())
            .sum()
    }
}
// end_submission
