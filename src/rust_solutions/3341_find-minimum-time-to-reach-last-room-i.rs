#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());

        let mut q = BinaryHeap::new();
        let mut h = HashSet::new();

        q.push(Reverse((0, (0, 0))));

        while let Some(Reverse((t, (i, j)))) = q.pop() {
            if i + 1 == n && j + 1 == m {
                return t;
            }

            if h.contains(&(i, j)) {
                continue;
            }
            h.insert((i, j));

            if i > 0 {
                q.push(Reverse((max(t, move_time[i - 1][j]) + 1, (i - 1, j))));
            }
            if j > 0 {
                q.push(Reverse((max(t, move_time[i][j - 1]) + 1, (i, j - 1))));
            }
            if i + 1 < n {
                q.push(Reverse((max(t, move_time[i + 1][j]) + 1, (i + 1, j))));
            }
            if j + 1 < m {
                q.push(Reverse((max(t, move_time[i][j + 1]) + 1, (i, j + 1))));
            }
        }

        unreachable!()
    }
}
// end_submission
