#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_sum = nums.iter().filter(|&&x| x % 2 == 0).sum::<i32>();

        let mut sums = Vec::new();

        for q in queries {
            let (q_v, q_i) = (q[0], q[1] as usize);


            if nums[q_i] % 2 == 0 {
                even_sum -= nums[q_i];
            }
            nums[q_i] += q_v;
            if nums[q_i] % 2 == 0 {
                even_sum += nums[q_i];
            }

            sums.push(even_sum);
        }

        sums
    }
}
// end_submission
