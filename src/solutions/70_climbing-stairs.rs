#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            dp[i] += dp[i - 1];
            if i > 1 {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }
}
// end_submission
