#[allow(unused)]
use crate::stubs::*;

struct Solution;

 // start_submission
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();

        for mut initial in [0, 1] {
            let mut v = Vec::new();

            for (w, &g) in words.iter().zip(groups.iter()) {
                if g != initial {
                    v.push(w.clone());
                    initial ^= 1;
                }
            }

            if v.len() > res.len() {
                res = v;
            }
        }

        res
    }
}
 // end_submission
