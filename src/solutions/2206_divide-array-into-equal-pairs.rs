#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut h = HashSet::new();

        for n in nums {
            if h.contains(&n) {
                h.remove(&n);
            } else {
                h.insert(n);
            }
        }

        h.is_empty()
    }
}
// end_submission
