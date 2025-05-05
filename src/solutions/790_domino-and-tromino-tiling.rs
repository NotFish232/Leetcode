#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    const MOD: i32 = 1_000_000_007;

    pub fn num_tilings(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 2];
        dp[1] = 1;
        dp[2] = 1;

        for i in 3..n as usize + 2 {
            dp[i] = ((2 * dp[i - 1]) % Self::MOD + dp[i - 3]) % Self::MOD;
        }

        dp[dp.len() - 1]
    }
}
// end_submission
