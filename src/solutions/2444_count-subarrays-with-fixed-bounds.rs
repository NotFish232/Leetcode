#[allow(unused)]
use crate::stubs::*;

struct Solution;

 // start_submission
 use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    fn solve_inclusive(nums: &[i32], min_k: i32, max_k: i32) ->  i64 {
        let mut res = 0;

        let (mut l, mut r) = (0, 0);

        let mut counts = HashMap::new();

        while r < nums.len() {
            *counts.entry(nums[r]).or_insert(0) += 1;

            while counts.contains_key(&min_k) && counts.contains_key(&max_k) {
                res += (nums.len() - r) as i64;

                if let Entry::Occupied(mut e) = counts.entry(nums[l]) {
                    *e.get_mut() -= 1;

                    if *e.get() == 0 {
                        e.remove_entry();
                    }
                }

                l += 1;
            }
            r += 1;
        }

        res
    }

    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0;

        let mut prev_i = 0;

        for i in 0..=nums.len() {
            if i == nums.len() || (nums[i] < min_k || nums[i] > max_k) {
                res += Self::solve_inclusive(&nums[prev_i..i], min_k, max_k);
                prev_i = i + 1;
            }
        }


        res
    }
}
 // end_submission
