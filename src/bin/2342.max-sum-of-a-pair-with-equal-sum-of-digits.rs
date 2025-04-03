#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<u32, i32> = HashMap::new();
        let mut max_sum = -1;

        for num in nums {
            let digits_sum = num
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum();

            if !m.contains_key(&digits_sum) {
                m.insert(digits_sum, num);
            } else {
                let saved_num = m.get(&digits_sum).unwrap();

                if saved_num + num > max_sum {
                    max_sum = saved_num + num;
                }

                if num > *saved_num {
                    m.insert(digits_sum, num);
                }
            }
        }

        max_sum
    }
}
// end_submission
