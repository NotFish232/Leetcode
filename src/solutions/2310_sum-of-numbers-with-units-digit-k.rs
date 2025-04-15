#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn minimum_numbers(mut num: i32, k: i32) -> i32 {
        if k == 0 {
            return if num % 10 == 0 {
                if num != 0 { 1 } else { 0 }
            } else {
                -1
            };
        }

        let mut c = 0;

        while num >= 0 {
            if num % 10 == 0 && (num == 0 || c != 0) {
                return c;
            }

            num -= k;
            c += 1;
        }

        -1
    }
}
// end_submission
