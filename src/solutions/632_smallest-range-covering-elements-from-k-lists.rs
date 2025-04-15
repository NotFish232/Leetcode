#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::{Reverse, max},
    collections::BinaryHeap,
};

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cur_range = (0, i32::MAX);
        let mut cur_max = i32::MIN;

        let mut heap = BinaryHeap::new();

        for i in 0..nums.len() {
            heap.push((Reverse(nums[i][0]), i, 0));
            cur_max = max(cur_max, nums[i][0]);
        }

        while !heap.is_empty() {
            let (Reverse(cur_min), p, i) = heap.pop().unwrap();

            if cur_max - cur_min < cur_range.1 - cur_range.0 {
                cur_range = (cur_min, cur_max);
            }
            if i + 1 < nums[p].len() {
                cur_max = max(cur_max, nums[p][i + 1]);
                heap.push((Reverse(nums[p][i + 1]), p, i + 1));
            } else {
                break;
            }
        }

        vec![cur_range.0, cur_range.1]
    }
}
// end_submission
