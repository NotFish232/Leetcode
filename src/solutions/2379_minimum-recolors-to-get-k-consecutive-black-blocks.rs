#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        blocks
            .as_bytes()
            .windows(k as usize)
            .map(|c| c.iter().filter(|&&x| x == b'W').count())
            .min()
            .unwrap() as i32
    }
}
// end_submission
