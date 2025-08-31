#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        let mut dp = vec![0.0; n as usize];
        dp[0] = 1.0;

        for i in 2..=n as usize {
            dp[i - 1] = ((i - 2) as f64 * dp[i - 2] + 1.0) / i as f64;
        }

        dp[n as usize - 1]
    }
}
// end_submission
