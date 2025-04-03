#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let partition_point_func = |&i: &usize| -> bool {
            let mut v = vec![0; nums.len() + 1];

            for q in queries.iter().take(i) {
                v[q[0] as usize] += q[2];
                v[q[1] as usize + 1] -= q[2];
            }

            let mut p_sum = 0;
            for j in 0..nums.len() {
                p_sum += v[j];

                if p_sum < nums[j] {
                    return true;
                }
            }

            false
        };

        let idxs: Vec<_> = (0..=queries.len()).collect();

        let point = idxs.partition_point(partition_point_func);

        if point != queries.len() + 1 {
            point as i32
        } else {
            -1
        }
    }
}
// end_submission
