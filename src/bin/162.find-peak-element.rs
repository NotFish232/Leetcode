#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::{self, Ordering};

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let search_fn = |&i: &usize| -> cmp::Ordering {
            let num = nums[i];
            let lhs = *nums.get(i - 1).unwrap_or(&i32::MIN);
            let rhs = *nums.get(i + 1).unwrap_or(&i32::MIN);

            if num >= lhs && num >= rhs {
                Ordering::Equal
            } else if lhs >= num {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        };

        let idxs: Vec<_> = (0..nums.len()).collect();

        idxs.binary_search_by(search_fn).unwrap() as i32
    }
}
// end_submission
