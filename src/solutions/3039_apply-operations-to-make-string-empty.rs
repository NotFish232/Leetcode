#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::Ordering, collections::HashMap};

impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut counts = HashMap::new();

        let mut max_count = 0;

        let mut res = String::new();

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;

            match counts[&c].cmp(&max_count) {
                Ordering::Greater => {
                    res.clear();
                    res.push(c);
                    max_count = counts[&c];
                }
                Ordering::Equal => {
                    res.push(c);
                }
                Ordering::Less => {}
            }
        }

        res
    }
}
// end_submission
