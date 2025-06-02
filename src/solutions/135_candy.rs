#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut sums = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                sums[i] = sums[i - 1] + 1;
            }
        }

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                sums[i] = sums[i + 1] + 1;
            }
        }

        sums.iter().sum()
    }
}
// end_submission
