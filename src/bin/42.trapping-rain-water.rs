impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left_ptr = 0;
        let mut right_ptr = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;

        while left_ptr < right_ptr {
            if height[left_ptr] < height[right_ptr] {
                if height[left_ptr] < left_max {
                    sum += left_max - height[left_ptr];
                } else {
                    left_max = height[left_ptr];
                }

                left_ptr += 1;
            } else {
                if height[right_ptr] < right_max {
                    sum += right_max - height[right_ptr];
                } else {
                    right_max = height[right_ptr];
                }

                right_ptr -= 1;
            }
        }

        sum
    }
}
