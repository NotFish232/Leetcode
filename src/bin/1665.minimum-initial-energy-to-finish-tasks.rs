#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_by(|a, b| (a[1] - a[0]).cmp(&(b[1] - b[0])));

        let mut count = 0;
        for x in tasks {
            count = x[1].max(x[0] + count);
        }

        count
    }
}
// end_submission
