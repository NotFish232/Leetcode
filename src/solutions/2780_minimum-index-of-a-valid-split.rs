#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut h = HashMap::new();

        for &n in &nums {
            *h.entry(n).or_insert(0) += 1;
        }

        let Some((&val, &freq)) = h.iter().max_by_key(|(_, v)| **v) else {
            panic!();
        };

        let mut val_count = 0;

        for i in 0..nums.len() {
            if nums[i] == val {
                val_count += 1;
            }

            if 2 * val_count > i + 1 && 2 * (freq - val_count) > nums.len() - i - 1 {
                return i as i32;
            }
        }

        -1
    }
}
// end_submission
