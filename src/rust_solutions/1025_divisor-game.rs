#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let mut v = vec![false; n as usize];

        'out: for x in 2..=n {
            for i in 1..=(x as f32).sqrt() as i32 {
                if x % i == 0 && !v[(x - i) as usize - 1] {
                    v[x as usize - 1] = true;
                    continue 'out;
                }
            }
            v[x as usize - 1] = false;
        }

        v[n as usize - 1]
    }
}
// end_submission
