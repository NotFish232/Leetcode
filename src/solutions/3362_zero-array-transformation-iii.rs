#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort();

        let mut pq = BinaryHeap::new();

        let mut q_idx = 0;
        let mut d = vec![0; nums.len() + 1];
        let mut count = 0;
        let mut cur = 0;

        for i in 0..nums.len() {
            while q_idx < queries.len() && queries[q_idx][0] == i as i32 {
                pq.push(queries[q_idx][1]);
                q_idx += 1;
            }

            cur += d[i];

            while cur < nums[i] {
                if !pq.is_empty() && **pq.peek().as_ref().unwrap() >= i as i32 {
                    cur += 1;
                    count += 1;
                    d[pq.pop().unwrap() as usize + 1] -= 1;
                } else {
                    return -1;
                }
            }
        }

        queries.len() as i32 - count
    }
}
// end_submission
