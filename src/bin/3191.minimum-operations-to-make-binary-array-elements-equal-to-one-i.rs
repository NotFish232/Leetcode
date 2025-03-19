impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;

                count += 1;
            }
        }

        if nums[nums.len() - 1] == 1 && nums[nums.len() - 2] == 1 {
            count
        } else {
            -1
        }
    }
}
