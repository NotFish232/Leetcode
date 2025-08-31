#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cur = 0;

        for i in 1..prices.len() {
            cur = max(cur, cur + prices[i] - prices[i - 1]);
        }

        cur
    }
}
// end_submission
