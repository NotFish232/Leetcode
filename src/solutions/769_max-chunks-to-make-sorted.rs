#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut starting_i = 0;
        let mut mask = 0;
        let mut count = 0;

        for (i, x) in arr.into_iter().enumerate() {
            mask |= 1 << x;

            if mask == 2i32.pow((i + 1) as u32) - 2i32.pow(starting_i as u32) {
                mask = 0;
                starting_i = i + 1;
                count += 1;
            }
        }

        count
    }
}
// end_submission
