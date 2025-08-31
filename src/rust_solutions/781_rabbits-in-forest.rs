#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut h = HashMap::new();

        for a in answers {
            *h.entry(a).or_insert(0) += 1;
        }

        let mut res = 0;

        for (num, count) in h {
            res += (num + 1) * ((count + num) / (num + 1));
        }

        res
    }
}
// end_submission
