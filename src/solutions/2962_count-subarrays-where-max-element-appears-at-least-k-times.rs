#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_val = *nums.iter().max().unwrap();

        let mut h = HashMap::new();

        let (mut l, mut r) = (0, 0);
        let mut count = 0;

        while r < nums.len() {
            *h.entry(nums[r]).or_insert(0) += 1;

            while *h.get(&max_val).unwrap_or(&0) >= k {
                count += (nums.len() - r) as i64;

                if let Entry::Occupied(mut e) = h.entry(nums[l]) {
                    *e.get_mut() -= 1;
                    if *e.get() == 0 {
                        e.remove_entry();
                    }
                }

                l += 1;
            }

            r += 1;
        }

        count
    }
}
// end_submission
