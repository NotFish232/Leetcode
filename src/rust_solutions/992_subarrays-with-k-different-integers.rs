#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    fn subbarrays_at_least_k(nums: &[i32], k: i32) -> i32 {
        let (mut l, mut r) = (0, 0);

        let mut h = HashMap::new();

        let mut count = 0;

        while r < nums.len() {
            *h.entry(nums[r]).or_insert(0) += 1;

            while h.len() as i32 == k {
                count += (nums.len() - r) as i32;

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

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subbarrays_at_least_k(&nums, k) - Self::subbarrays_at_least_k(&nums, k + 1)
    }
}
// end_submission
