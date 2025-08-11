#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();

        let n = nums.len() / 3;

        let mut mins = vec![0; nums.len()];
        let mut maxs = vec![0; nums.len()];

        let mut h = BinaryHeap::new();
        let mut p_sum = 0;
        for i in 0..3 * n {
            if h.len() < n {
                p_sum += nums[i];
                h.push(nums[i]);
            } else {
                if nums[i] < *h.peek().unwrap() {
                    p_sum -= h.pop().unwrap();
                    h.push(nums[i]);
                    p_sum += nums[i];
                }
            }

            mins[i] = p_sum;
        }

        let mut h = BinaryHeap::new();
        let mut p_sum = 0;
        for i in (0..3 * n).rev() {
            if h.len() < n {
                p_sum += nums[i];
                h.push(Reverse(nums[i]));
            } else {
                if nums[i] > h.peek().unwrap().0 {
                    p_sum -= h.pop().unwrap().0;
                    h.push(Reverse(nums[i]));
                    p_sum += nums[i];
                }
            }

            maxs[i] = p_sum;
        }

        (n - 1..2 * n).map(|i| mins[i] - maxs[i + 1]).min().unwrap()
    }
}
// end_submission
