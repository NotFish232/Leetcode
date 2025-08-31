#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|x| x[0]);

        let mut new_intervals: Vec<Vec<_>> = Vec::new();

        for interval in intervals {
            if new_intervals.is_empty() || new_intervals[new_intervals.len() - 1][1] < interval[0] {
                new_intervals.push(interval);
            } else {
                let old_interval = new_intervals.pop().unwrap();
                new_intervals.push(vec![old_interval[0], max(old_interval[1], interval[1])])
            }
        }

        new_intervals
    }
}
// end_submission
