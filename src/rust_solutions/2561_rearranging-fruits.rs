#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::min, collections::HashMap};

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let (mut freq1, mut freq2, mut total_freq) =
            (HashMap::new(), HashMap::new(), HashMap::new());
        for &b in &basket1 {
            *freq1.entry(b).or_insert(0) += 1;
            *total_freq.entry(b).or_insert(0) += 1;
        }
        for &b in &basket2 {
            *freq2.entry(b).or_insert(0) += 1;
            *total_freq.entry(b).or_insert(0) += 1;
        }

        if total_freq.values().any(|&v| v % 2 == 1) {
            return -1;
        }

        let (mut excess1, mut excess2) = (Vec::new(), Vec::new());
        for (b, c) in &freq1 {
            let diff = (c - *freq2.get(b).unwrap_or(&0)) / 2;

            for _ in 0..diff {
                excess1.push(*b);
            }
        }
        for (b, c) in &freq2 {
            let diff = (c - *freq1.get(b).unwrap_or(&0)) / 2;

            for _ in 0..diff {
                excess2.push(*b);
            }
        }

        excess1.sort();
        excess2.sort();

        let min_el = min(
            *basket1.iter().min().unwrap(),
            *basket2.iter().min().unwrap(),
        );

        let mut cost = 0;

        while !excess1.is_empty() {
            if excess1[0] < excess2[0] {
                cost += min(excess1.remove(0), 2 * min_el) as i64;
                excess2.pop();
            } else {
                cost += min(excess2.remove(0), 2 * min_el) as i64;
                excess1.pop();
            }
        }

        cost
    }
}
// end_submission
