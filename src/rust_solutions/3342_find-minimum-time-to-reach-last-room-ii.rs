#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());

        let mut q = BinaryHeap::new();
        let mut v = vec![vec![false; m]; n];

        q.push(Reverse((0, 0, (0, 0))));

        while let Some(Reverse((t, c, (i, j)))) = q.pop() {
            if i + 1 == n && j + 1 == m {
                return t;
            }

            if v[i][j] {
                continue;
            }
            v[i][j] = true;

            let d = if c % 2 == 0 { 1 } else { 2 };

            if i > 0 {
                q.push(Reverse((
                    max(t, move_time[i - 1][j]) + d,
                    c + 1,
                    (i - 1, j),
                )));
            }
            if j > 0 {
                q.push(Reverse((
                    max(t, move_time[i][j - 1]) + d,
                    c + 1,
                    (i, j - 1),
                )));
            }
            if i + 1 < n {
                q.push(Reverse((
                    max(t, move_time[i + 1][j]) + d,
                    c + 1,
                    (i + 1, j),
                )));
            }
            if j + 1 < m {
                q.push(Reverse((
                    max(t, move_time[i][j + 1]) + d,
                    c + 1,
                    (i, j + 1),
                )));
            }
        }

        unreachable!()
    }
}
// end_submission
