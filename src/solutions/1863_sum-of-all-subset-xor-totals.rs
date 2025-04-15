#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn solve(c: i32, idx: usize, nums: &[i32]) -> i32 {
        if idx == nums.len() {
            return c;
        }

        Self::solve(c, idx + 1, nums) + Self::solve(c ^ nums[idx], idx + 1, nums)
    }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::solve(0, 0, &nums)
    }
}
// end_submission
