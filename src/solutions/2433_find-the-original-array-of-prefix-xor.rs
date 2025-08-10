#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut v = vec![pref[0]; pref.len()];

        for i in 1..pref.len() {
            v[i] = pref[i] ^ pref[i - 1];
        }

        v
    }
}
// end_submission
