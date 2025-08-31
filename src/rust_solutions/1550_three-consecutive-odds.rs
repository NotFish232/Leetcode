#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|w| w.iter().all(|x| x % 2 == 1))
    }
}
// end_submission
