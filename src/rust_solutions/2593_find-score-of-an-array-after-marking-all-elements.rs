#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut v: Vec<_> = nums.into_iter().enumerate().collect();
        v.sort_by(|(_, a), (_, b)| a.cmp(b));

        let mut b = vec![false; v.len()];

        let mut score: i64 = 0;

        for &(i, x) in &v {
            if b[i] {
                continue;
            }

            score += x as i64;

            b[i] = true;
            if i > 0 {
                b[i - 1] = true;
            }
            if i + 1 < v.len() {
                b[i + 1] = true;
            }
        }

        score
    }
}
// end_submission
