#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut h = HashMap::new();

        for x in 1..=n {
            let s = x
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .sum::<i32>();

            *h.entry(s).or_insert(0) += 1;
        }

        let max_val = h.values().max().unwrap();

        h.values().filter(|&v| v == max_val).count() as i32
    }
}
// end_submission
