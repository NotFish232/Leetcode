#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn _construct_distanced_sequence(
        v: &mut Vec<i32>,
        nums: &mut Vec<i32>,
        idx: usize,
        mut seen_1: bool,
    ) -> bool {
        if nums.is_empty() {
            return true;
        }

        for i in idx..v.len() {
            for j in 0..nums.len() {
                let n = nums[j];
                let k = i + n as usize;

                if v[i] == 1 && k < v.len() && v[k] == 1 {
                    v[i] = n;
                    v[k] = n;
                    nums.remove(j);

                    if Solution::_construct_distanced_sequence(v, nums, i + 1, seen_1) {
                        return true;
                    }

                    v[i] = 1;
                    v[k] = 1;
                    nums.insert(j, n);
                }
            }

            if v[i] == 1 {
                if seen_1 {
                    break;
                } else {
                    seen_1 = true;
                }
            }
        }

        false
    }

    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut v: Vec<i32> = vec![1; (2 * n - 1) as usize];
        let mut nums: Vec<i32> = (2..=n).rev().collect();

        Solution::_construct_distanced_sequence(&mut v, &mut nums, 0, false);

        v
    }
}
// end_submission
