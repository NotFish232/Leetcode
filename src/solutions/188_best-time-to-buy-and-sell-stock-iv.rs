#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut buys = vec![-prices[0]; k as usize];
        let mut sells = vec![0; k as usize];

        for i in 1..prices.len() {
            for j in 0..k as usize {
                if j == 0 {
                    buys[j] = max(buys[j], -prices[i]);
                } else {
                    buys[j] = max(buys[j], sells[j - 1] - prices[i]);
                }
                sells[j] = max(sells[j], buys[j] + prices[i]);
            }
        }

        sells.into_iter().max().unwrap()
    }
}
// end_submission
