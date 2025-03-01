impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut num_zeros = 0;
        while i < nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] == 0 {
                num_zeros += 1;
                nums.remove(i);
                i -= 1;
            }

            i += 1;
        }

        for _ in 0..num_zeros {
            nums.push(0);
        }

        nums
    }
}
