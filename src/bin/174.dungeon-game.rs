#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cmp::{max, min};

#[allow(dead_code)]
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();

        let mut dp = vec![vec![i32::MAX; n]; m];
        dp[m - 1][n - 1] = max(-dungeon[m - 1][n - 1], 0);

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i + 1 < m {
                    dp[i][j] = min(dp[i][j], max(dp[i + 1][j] - dungeon[i][j], 0));
                }
                if j + 1 < n {
                    dp[i][j] = min(dp[i][j], max(dp[i][j + 1] - dungeon[i][j], 0));
                }
            }
        }

        dp[0][0] + 1
    }
}
// end_submission
