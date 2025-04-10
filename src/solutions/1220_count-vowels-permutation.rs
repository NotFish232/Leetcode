#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let modulo = 1_000_000_007;

        let mut dp = vec![vec![0i64; 5]; n as usize];

        for i in 0..5 {
            dp[0][i] = 1;
        }

        for i in 1..n as usize {
            dp[i][1] += dp[i - 1][0];

            dp[i][0] += dp[i - 1][1];
            dp[i][2] += dp[i - 1][1];

            dp[i][0] += dp[i - 1][2];
            dp[i][1] += dp[i - 1][2];
            dp[i][3] += dp[i - 1][2];
            dp[i][4] += dp[i - 1][2];

            dp[i][2] += dp[i - 1][3];
            dp[i][4] += dp[i - 1][3];

            dp[i][0] += dp[i - 1][4];

            for j in 0..5 {
                dp[i][j] %= modulo;
            }
        }

        dp[n as usize - 1]
            .iter()
            .fold(0, |acc, x| (acc + x) % modulo) as i32
    }
}
// end_submission
