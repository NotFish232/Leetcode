#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let (mut l_ptr, mut r_ptr) = (0, 0);

        let mut count = 0;

        let mut val_counts = HashMap::new();
        let mut pair_count = 0;

        while r_ptr < nums.len() {
            pair_count += val_counts.get(&nums[r_ptr]).unwrap_or(&0);
            *val_counts.entry(nums[r_ptr]).or_insert(0) += 1;

            while pair_count >= k {
                count += (nums.len() - r_ptr) as i64;
                *val_counts.entry(nums[l_ptr]).or_insert(0) -= 1;
                pair_count -= val_counts.get(&nums[l_ptr]).unwrap_or(&0);

                l_ptr += 1;
            }

            r_ptr += 1;
        }

        count
    }
}
// end_submission
