#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            let comp: i32 = target - num;
            if m.contains_key(&comp) {
                return vec![*m.get(&comp).unwrap(), idx as i32];
            }
            m.insert(num, idx as i32);
        }

        vec![]
    }
}
// end_submission
