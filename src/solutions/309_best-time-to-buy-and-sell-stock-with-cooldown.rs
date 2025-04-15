#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 3]; prices.len()];
        dp[0][0] = -prices[0];
        // 0 -> buy, 1 -> sell, 2 -> cooldown

        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][2] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = max(dp[i - 1][2], dp[i - 1][1]);
        }

        max(dp[prices.len() - 1][1], dp[prices.len() - 1][2])
    }
}
// end_submission
