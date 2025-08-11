#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}
// end_submission
