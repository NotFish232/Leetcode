#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut idx_1 = 0;
        let mut idx_2 = 0;
        let total_len = nums1.len() + nums2.len();

        loop {
            let num_1_val = match nums1.get(idx_1) {
                Some(&n) => n,
                None => i32::MAX,
            };
            let num_2_val = match nums2.get(idx_2) {
                Some(&n) => n,
                None => i32::MAX,
            };

            if idx_1 + idx_2 == (total_len - 1) / 2 {
                return if total_len % 2 == 0 {
                    (min(num_1_val, num_2_val)
                        + min(
                            max(num_1_val, num_2_val),
                            min(
                                match nums1.get(idx_1 + 1) {
                                    Some(&n) => n,
                                    None => i32::MAX,
                                },
                                match nums2.get(idx_2 + 1) {
                                    Some(&n) => n,
                                    None => i32::MAX,
                                },
                            ),
                        )) as f64
                        / 2f64
                } else {
                    min(num_1_val, num_2_val) as f64
                };
            }

            if num_1_val <= num_2_val {
                idx_1 += 1
            } else {
                idx_2 += 1
            };
        }
    }
}
// end_submission
