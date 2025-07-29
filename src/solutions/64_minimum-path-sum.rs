#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut dp = vec![vec![i32::MAX; m]; n];
        dp[0][0] = grid[0][0];

        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    dp[i][j] = min(dp[i][j], dp[i - 1][j] + grid[i][j]);
                }
                if j > 0 {
                    dp[i][j] = min(dp[i][j], dp[i][j - 1] + grid[i][j]);
                }
            }
        }

        dp[n - 1][m - 1]
    }
}
// end_submission
