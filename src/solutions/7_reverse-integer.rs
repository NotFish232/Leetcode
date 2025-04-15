#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut val = 0i64;
        let is_negative = x < 0;
        x = x.abs();
        while x > 0 {
            val *= 10;
            val += (x % 10) as i64;
            x /= 10;
        }
        if val > i32::MAX as i64 {
            0
        } else {
            ((if is_negative { -1 } else { 1 }) * val) as i32
        }
    }
}
// end_submission
