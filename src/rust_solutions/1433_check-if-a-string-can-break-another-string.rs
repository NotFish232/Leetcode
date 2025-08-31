#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut v1: Vec<_> = s1.chars().collect();
        let mut v2: Vec<_> = s2.chars().collect();
        v1.sort();
        v2.sort();

        v1.iter().zip(v2.iter()).all(|(&a, &b)| a <= b)
            || v2.iter().zip(v1.iter()).all(|(&a, &b)| a <= b)
    }
}
// end_submission
