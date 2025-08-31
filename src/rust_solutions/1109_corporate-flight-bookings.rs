#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut diffs = vec![0; n as usize + 1];
        for b in bookings {
            diffs[b[0] as usize - 1] += b[2];
            diffs[b[1] as usize] -= b[2];
        }

        let mut v = Vec::new();
        let mut ps = 0;
        for i in 0..n as usize {
            ps += diffs[i];
            v.push(ps);
        }

        v
    }
}
// end_submission
