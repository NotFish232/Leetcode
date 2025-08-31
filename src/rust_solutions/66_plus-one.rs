#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();

        digits.insert(0, 0);
        digits[n] += 1;

        for i in (0..n).rev() {
            digits[i] += digits[i + 1] / 10;
            digits[i + 1] %= 10;
        }

        if digits.first() == Some(&0) {
            digits.remove(0);
        }

        digits
    }
}
// end_submission
