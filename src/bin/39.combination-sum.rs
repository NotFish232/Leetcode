impl Solution {
    fn _combination_sum(
        current: &mut Vec<i32>,
        nums: &mut Vec<i32>,
        target: i32,
        idx: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }

        for i in idx..nums.len() {
            if nums[i] <= target {
                current.push(nums[i]);
                Self::_combination_sum(current, nums, target - nums[i], i, result);

                current.pop();
            }
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::_combination_sum(&mut Vec::new(), &mut candidates, target, 0, &mut result);

        result
    }
}
