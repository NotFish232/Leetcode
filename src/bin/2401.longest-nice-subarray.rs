#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut c = nums[0];
        let mut max_count = 1;

        while r < nums.len() {
            while c & nums[r] != 0 {
                c ^= nums[l];
                l += 1;
            }

            c |= nums[r];

            max_count = max(max_count, (r - l + 1) as i32);

            r += 1;
        }

        max_count
    }
}
// end_submission
