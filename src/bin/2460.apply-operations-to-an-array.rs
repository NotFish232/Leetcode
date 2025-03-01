impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![0; nums.len()];

        let mut v_i = 0;
        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }

            if nums[i] != 0 {
                v[v_i] = nums[i];
                v_i += 1;
            }
        }

        v
    }
}
