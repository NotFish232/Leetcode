#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::new();

        for num in nums {
            h.push(num);
        }

        for _ in 0..(k - 1) as usize {
            h.pop();
        }

        h.pop().unwrap()
    }
}
// end_submission
