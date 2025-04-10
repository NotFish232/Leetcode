#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s_copy = s.to_string();

        while let Some(idx) = s_copy.find(part.as_str()) {
            for _ in 0..part.len() {
                s_copy.remove(idx);
            }
        }

        s_copy
    }
}
// end_submission
