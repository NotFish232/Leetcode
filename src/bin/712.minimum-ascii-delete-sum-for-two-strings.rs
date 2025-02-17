use std::cmp::{max_by, min};

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for (i, ch) in s1.chars().enumerate() {
            dp[i + 1][0] = dp[i][0] + ch as i32;
        }
        for (i, ch) in s2.chars().enumerate() {
            dp[0][i + 1]= dp[0][i] + ch as i32;
        }

        for (i, ch1) in s1.chars().enumerate() {
            for (j, ch2) in s2.chars().enumerate() {
                if ch1 == ch2 {
                    dp[i + 1][j + 1] = dp[i][j];
                } else {
                    dp[i + 1][j + 1] = min(dp[i + 1][j] + ch2 as i32, dp[i][j + 1] + ch1 as i32);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}