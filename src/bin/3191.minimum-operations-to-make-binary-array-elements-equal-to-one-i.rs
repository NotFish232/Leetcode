impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                if i + 2 >= nums.len() {
                    return -1;
                }

                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;

                count += 1;
            }
        }

        count
    }
}
