use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = height.len() - 1;
        let mut max_area = i32::MIN;

        while (left_ptr < right_ptr) {
            let area = (right_ptr - left_ptr) as i32 * min(height[left_ptr], height[right_ptr]);

            max_area = max(max_area, area);

            if (height[left_ptr] < height[right_ptr]) {
                left_ptr += 1;
            } else {
                right_ptr -= 1;
            }
        }

        max_area
    }
}
