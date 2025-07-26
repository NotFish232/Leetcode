#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut v = Vec::from([0]);

        for i in 0..n {
            for j in (0..v.len()).rev() {
                v.push(v[j] | 1 << i);
            }
        }

        v
    }
}
// end_submission
