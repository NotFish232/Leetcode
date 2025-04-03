#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut prev = Vec::new();
        let mut post = Vec::new();
        let mut pivot_count = 0;

        for num in nums {
            if num < pivot {
                prev.push(num);
            } else if num > pivot {
                post.push(num);
            } else {
                pivot_count += 1;
            }
        }

        prev.extend(vec![pivot; pivot_count]);
        prev.extend(post);

        prev
    }
}
// end_submission
