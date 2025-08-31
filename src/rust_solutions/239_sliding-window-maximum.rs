#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q = VecDeque::new();
        let mut v = Vec::new();

        for i in 0..nums.len() {
            while let Some(&n) = q.back() {
                if n < nums[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(nums[i]);

            if i >= k as usize && q[0] == nums[i - k as usize] {
                q.pop_front();
            }

            if i + 1 >= k as usize {
                v.push(q[0]);
            }
        }

        v
    }
}
// end_submission
