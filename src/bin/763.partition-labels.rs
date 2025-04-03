#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::{cmp::max, collections::HashMap};

#[allow(dead_code)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut first_occur = HashMap::new();
        let mut last_occur = HashMap::new();

        for (i, ch) in s.chars().enumerate() {
            first_occur.entry(ch).or_insert(i);
            last_occur.insert(ch, i);
        }

        let mut intervals: Vec<_> = first_occur
            .iter()
            .map(|(&k, &v)| (v, *last_occur.get(&k).unwrap()))
            .collect();

        intervals.sort();

        let mut new_intervals: Vec<(usize, usize)> = Vec::new();

        for (a, b) in intervals {
            let idx = new_intervals.len() - 1;
            if new_intervals.is_empty() || new_intervals[idx].1 < a {
                new_intervals.push((a, b));
            } else {
                new_intervals[idx].1 = max(new_intervals[idx].1, b);
            }
        }

        new_intervals
            .iter()
            .map(|&(a, b)| (b - a + 1) as i32)
            .collect()
    }
}
// end_submission
