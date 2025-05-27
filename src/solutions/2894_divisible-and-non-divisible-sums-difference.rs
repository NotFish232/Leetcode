#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        (0..=n).map(|x| if x % m != 0 { x } else { -x }).sum()
    }
}
// end_submission
