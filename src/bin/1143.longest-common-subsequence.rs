use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for (i, ch1) in text1.chars().enumerate() {
            for (j, ch2) in text2.chars().enumerate() {
                if ch1 == ch2 {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}
