#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let negative = n < 0;
        let mut n = (n as i64).abs();

        let mut res = 1.0;

        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }
            x *= x;
            n /= 2;
        }

        if negative { 1.0 / res } else { res }
    }
}
// end_submission
