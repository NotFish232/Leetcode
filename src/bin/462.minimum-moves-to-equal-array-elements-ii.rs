#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        (0..nums.len())
            .map(|i| (nums[i] - nums[nums.len() / 2]).abs())
            .sum()
    }
}
// end_submission
