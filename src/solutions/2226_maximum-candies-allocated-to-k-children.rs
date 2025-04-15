#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let max_candies = candies.iter().map(|&i| i as i64).sum::<i64>() / k;

        let partition_point_fn = |&c: &i64| -> bool {
            if c == 0 {
                return true;
            }

            let mut count = 0i64;
            for &candy in &candies {
                count += candy as i64 / c;
            }

            count >= k
        };

        let possible_candies: Vec<_> = (0..=max_candies).collect();

        possible_candies.partition_point(partition_point_fn) as i32 - 1
    }
}
// end_submission
