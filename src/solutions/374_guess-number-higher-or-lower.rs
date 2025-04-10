#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

fn guess(_: i32) -> i32 {
    0
}

// start_submission
#[allow(dead_code)]
impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        loop {
            let mid = l + (r - l) / 2;

            match guess(mid) {
                -1 => r = mid - 1,
                1 => l = mid + 1,
                _ => break mid,
            }
        }
    }
}
// end_submission
