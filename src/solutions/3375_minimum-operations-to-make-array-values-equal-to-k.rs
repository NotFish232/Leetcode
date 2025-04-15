#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = HashSet::new();
        let mut count = 0;

        for n in nums {
            if !h.contains(&n) {
                if n < k {
                    return -1;
                }
                if n > k {
                    h.insert(n);
                    count += 1;
                }
            }
        }

        count
    }
}
// end_submission
