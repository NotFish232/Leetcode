#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut h = HashMap::new();

        for (i, g) in group_sizes.into_iter().enumerate() {
            let entry: &mut Vec<Vec<i32>> = h.entry(g).or_insert(Vec::new());

            if entry.is_empty() || entry[entry.len() - 1].len() == g as usize {
                entry.push(Vec::new());
            }

            let l = entry.len();
            entry[l - 1].push(i as i32);
        }

        h.values().flatten().cloned().collect()
    }
}
// end_submission
