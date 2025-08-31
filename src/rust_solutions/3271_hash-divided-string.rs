#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(k as usize)
            .map(|x| {
                (b'a' + (x.iter().map(|&c| c as u16 - 'a' as u16).sum::<u16>() % 26) as u8) as char
            })
            .collect()
    }
}
// end_submission
