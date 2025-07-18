#[allow(unused)]
use crate::stubs::*;

struct Solution;

 // start_submission
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}
 // end_submission
