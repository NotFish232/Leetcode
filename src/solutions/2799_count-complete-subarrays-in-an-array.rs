#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::{hash_map::Entry, HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut vals = HashSet::new();

        for &n in &nums {
            vals.insert(n);
        }

        let mut count = 0;

        let (mut l, mut r) = (0, 0);

        let mut seen = HashMap::new();

        while r < nums.len() {
            *seen.entry(nums[r]).or_insert(0) += 1;

            while vals.len() == seen.len() {
                count += (nums.len() - r) as i32;

                if let Entry::Occupied(mut e) = seen.entry(nums[l]) {
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
