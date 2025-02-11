impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut closest_dist = i32::MAX;
        let mut closest_sum = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left_ptr = i + 1;
            let mut right_ptr = nums.len() - 1;

            while left_ptr < right_ptr {
                let sum = nums[i] + nums[left_ptr] + nums[right_ptr];
                let dist = (sum - target).abs();

                if dist < closest_dist {
                    closest_dist = dist;
                    closest_sum = sum;
                }

                if (sum > target) {
                    right_ptr -= 1;
                } else if (sum < target) {
                    left_ptr += 1;
                } else {
                    return target;
                }
            }
        }

        closest_sum
    }
}
