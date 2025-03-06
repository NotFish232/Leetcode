use std::cmp::max;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut sells = vec![-prices[0]; k as usize];
        let mut buys = vec![0; k as usize];

        for i in 1..prices.len() {
            for j in 0..k as usize {
                if j == 0 {
                    sells[j] = max(sells[j], -prices[i]);
                } else {
                    sells[j] = max(sells[j], buys[j - 1] - prices[i]);
                }
                buys[j] = max(buys[j], sells[j] + prices[i]);
            }
        }

        buys.into_iter().max().unwrap()
    }
}
