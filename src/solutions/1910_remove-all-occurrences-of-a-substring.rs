#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
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
