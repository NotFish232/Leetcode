#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() - 2 {
            count += (2 * (nums[i] + nums[i + 2]) == nums[i + 1]) as i32;
        }

        count
    }
}
// end_submission
