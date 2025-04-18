#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        word1
            .chars()
            .zip(word2.chars())
            .fold([0i32; 26], |mut v, (ch1, ch2)| {
                v[ch1 as usize - 'a' as usize] += 1;
                v[ch2 as usize - 'a' as usize] -= 1;
                v
            })
            .into_iter()
            .all(|x| x.abs() <= 3)
    }
}
// end_submission
