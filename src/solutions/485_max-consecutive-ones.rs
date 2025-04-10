#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut cur_count = 0;

        for n in nums.into_iter().chain([0]) {
            if n == 1 {
                cur_count += 1;
            } else {
                max_count = max(max_count, cur_count);
                cur_count = 0;
            }
        }

        max_count
    }
}
// end_submission
