#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::{max, min};

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut v = vec![0; questions.len() + 1];

        for (i, q) in questions.iter().enumerate() {
            let j = min(i + q[1] as usize + 1, questions.len());

            v[i + 1] = max(v[i + 1], v[i]);
            v[j] = max(v[j], v[i] + q[0] as i64);
        }

        v[questions.len()]
    }
}
// end_submission
