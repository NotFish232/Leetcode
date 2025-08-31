#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        (0..nums.len() - 2)
            .filter(|&i| 2 * (nums[i] + nums[i + 2]) == nums[i + 1])
            .count() as i32
    }
}
// end_submission
