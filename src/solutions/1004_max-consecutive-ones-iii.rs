#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut zero_count = 0;
        let mut max_count = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                zero_count += 1;
            }

            if zero_count > k {
                zero_count -= 1;
                while nums[l] != 0 {
                    l += 1;
                }
                l += 1;
            }

            max_count = max(max_count, (r - l + 1) as i32);

            r += 1;
        }

        max_count
    }
}
// end_submission
