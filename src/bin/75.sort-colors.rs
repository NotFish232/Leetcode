#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut count_0 = 0;
        let mut count_1 = 0;

        for num in nums.iter() {
            if *num == 0 {
                count_0 += 1;
            } else if *num == 1 {
                count_1 += 1;
            }
        }

        nums.fill(2);

        for i in 0..count_0 + count_1 {
            if i < count_0 {
                nums[i] = 0;
            } else {
                nums[i] = 1;
            }
        }
    }
}
// end_submission
