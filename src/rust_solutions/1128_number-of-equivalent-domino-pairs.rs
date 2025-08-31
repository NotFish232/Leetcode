#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut h = HashMap::new();
        let mut c = 0;

        for mut d in dominoes {
            d.sort();

            c += *h.get(&d).unwrap_or(&0);
            *h.entry(d).or_insert(0) += 1;
        }

        c
    }
}
// end_submission
