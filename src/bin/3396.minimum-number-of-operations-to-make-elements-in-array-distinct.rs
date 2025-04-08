#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut h = HashSet::new();

        for (i, &n) in nums.iter().enumerate().rev() {
            if h.contains(&n) {
                return i as i32 / 3 + 1;
            } else {
                h.insert(n);
            }
        }

        0
    }
}
// end_submission
