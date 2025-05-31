#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        let mut q = VecDeque::new();
        let mut seen = HashMap::new();

        q.push_back((0, 0));

        while let Some((i, c)) = q.pop_front() {
            if *seen.get(&i).unwrap_or(&i32::MAX) <= c {
                continue;
            }
            seen.insert(i, c);

            for mut j in i + 1..min(i + 7, n * n) {
                let row = n - 1 - j / n;
                let col = if row % 2 != n % 2 {
                    j % n
                } else {
                    n - 1 - j % n
                };

                if board[row][col] != -1 {
                    j = board[row][col] as usize - 1;
                }

                q.push_back((j, c + 1));
            }
        }

        *seen.get(&(n * n - 1)).unwrap_or(&-1)
    }
}
// end_submission
