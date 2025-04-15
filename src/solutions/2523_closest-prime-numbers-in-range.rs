#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut v = vec![true; right as usize + 1];
        v[1] = false;

        let mut n = 2;
        while n * n <= right as usize {
            if v[n] {
                let mut m = n * n;
                while m <= right as usize {
                    v[m] = false;
                    m += n;
                }
            }

            n += 1;
        }

        let mut cur_dist = i32::MIN;
        let mut min_dist = i32::MAX;
        let (mut a, mut b) = (-1, -1);

        for i in left as usize..=right as usize {
            if v[i] {
                if cur_dist < 0 {
                    cur_dist = 1;
                    continue;
                }

                if cur_dist < min_dist {
                    min_dist = cur_dist;
                    a = i as i32 - cur_dist;
                    b = i as i32;
                }
                cur_dist = 1;
            } else {
                cur_dist += 1;
            }
        }

        vec![a, b]
    }
}
// end_submission
