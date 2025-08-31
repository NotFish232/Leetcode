#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use itertools::Itertools;
use std::cmp::min;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let [a, b, c] = [a, b, c].into_iter().sorted().collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        let min_moves = if a + 1 == b && b + 1 == c {
            0
        } else if min(b - a, c - b) <= 2 {
            1
        } else {
            2
        };
        let max_moves = c - a - 2;

        vec![min_moves, max_moves]
    }
}
// end_submission
