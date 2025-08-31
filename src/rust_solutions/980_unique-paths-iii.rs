#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use itertools::repeat_n;
use std::collections::VecDeque;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let (n, m) = (grid.len(), grid[0].len());

        let starting_s = (0..n)
            .flat_map(|i| repeat_n(i, m).zip(0..m))
            .find(|&(r, c)| grid[r][c] == 1)
            .unwrap();

        let empty_mask: usize = (0..n)
            .flat_map(|i| repeat_n(i, m).zip(0..m))
            .map(|(r, c)| if grid[r][c] == 0 { 1 << (r * m + c) } else { 0 })
            .sum();

        let mut q = VecDeque::new();
        q.push_back((starting_s, 0));

        let mut count = 0;

        while let Some(((r, c), mask)) = q.pop_front() {
            for (d_r, d_c) in directions {
                let new_r = r as i32 + d_r;
                let new_c = c as i32 + d_c;

                if 0 <= new_r && new_r < n as i32 && 0 <= new_c && new_c < m as i32 {
                    let new_r = new_r as usize;
                    let new_c = new_c as usize;

                    if grid[new_r][new_c] == 2 {
                        if mask == empty_mask {
                            count += 1;
                        }
                    }
                    if grid[new_r][new_c] == 0 && mask & (1 << (new_r * m + new_c)) == 0 {
                        let new_mask = mask + (1 << (new_r * m + new_c));
                        q.push_back(((new_r, new_c), new_mask));
                    }
                }
            }
        }

        count
    }
}
// end_submission
